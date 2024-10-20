const { Client: a } = require('irc-framework'),
    b = require('node-fetch'),
    c = require('dotenv');
c.config();

const d = new a(),
    e = '##drugs';

// Connect to the IRC server
d.connect({
    host: 'irc.libera.chat',
    port: 6667,
    nick: 'psypup',
    username: 'psypup',
    realname: 'PsyPup Harm Redux Bot'
});

// Once registered, join the desired channel
d.on('registered', () => {
    d.join(e);
    console.log(`Joined channel ${e}`);
});

// Listen for messages
d.on('message', async (a) => {
    const { message: c, target: f, nick: g } = a;

    // Check if the message is in the desired channel and mentions the bot's username
    if (f === e && c.includes('psypup')) {
        const query = c.replace(/psypup[:,]?/i, '').trim(); // Remove bot's name and get the rest of the message
        
        if (query.length > 0) {
            // Make the request to your PsyAI service
            const response = await (async (a, model = 'openai-next', temperature = 0, tokens = 100) => {
                try {
                    return await b(`${process.env.BASE_URL_BETA}/q`, {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({
                            question: `${a}\n\nRespond in a single sentence.`,
                            temperature,
                            tokens,
                            drug: false,
                            model,
                            version: 'v2'
                        })
                    });
                } catch (error) {
                    console.error(`Error in fetchQuestionFromPsyAI: ${error}`);
                    return null;
                }
            })(query);
            
            if (response) {
                const result = await response.json();
                d.say(f, `${g}: ${result.assistant}`);
            } else {
                d.say(f, `${g}: Sorry, I couldn't process that request.`);
            }
        } else {
            d.say(f, `${g}: Please provide a query after tagging me.`);
        }
    }
});

// Handle IRC errors
d.on('error', (a) => {
    console.error('IRC Error:', a);
});
