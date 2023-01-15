# trader
Advanced Programming Project Trader

## Arbitrager bot
Every time you want to start the bot, you have to create a new "Arbitrager" passing and EUR quantity, and then call "arbitrage" function to do only 1 arbitrage.

Its goal is to maximize only EUR through arbitrage over USD, YEN, YUAN, and over all three markets

An arbitrage is done selling EUR in a market and rebuy as much as possible in another market. If that seems not convenient it doesn't make any arbitrage and the function returns None.

Is not guaranteed that it doesn't convenient arbitrages because markets can change price after a trade or lock.

