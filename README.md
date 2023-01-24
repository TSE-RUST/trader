# trader
Advanced Programming Project Trader

## Arbitrager bot
Every time you want to start the bot, you have to create a new "Arbitrager" passing and EUR quantity, and then call "arbitrage" function to do only 1 arbitrage.

Its goal is to maximize only EUR through arbitrage over USD, YEN, YUAN, and over all three markets

An arbitrage is done selling EUR in a market and rebuy as much as possible in another market. If none arbitrage seems convenient it doesn't make any arbitrage and the function returns None as last parameter (it doesn nothing).

Is not guaranteed that it doesn't convenient arbitrages because markets can change price after a trade or lock.

# In detail:
Assumption: For price I mean spot price (e.g. if 2 EUR corresponds to 1 USD, then USD/EUR=2/1=2)
For each pair or market, and for each good (not EUR) it compute the profit gained from 1 arbitrage.
Then if the maximum profit is > 0, it execute the corresponding arbitrage.
Let's see how a profit can be computed:

Let's say that im looking to buy (the good bought is not important here) from the market A, I will now compare 3 market where I can sell (B, C and D).

![](assets/a.png)

## Step 1
First of all I need to know (only for the buy_market) the maximum quantity in EUR I can send to the buy_market (in the code is called "max_eur_to_send"), I will use the function get_buy_price, but only need to specify le quantity of good that I'm looking for, so I start requesting an infite quantity, then the market will tell me the maximum quantity to request, and recall the function again until the eur to be sent are more than the ones I have, and every time I divide by 2 the quantity. The quantity of good is called "max_alt_to_receive" (alt means altcoin, aka good).

## Step 2
I will define for each market 2 variables: min_price and max_price.

ONLY in the case of the buy_market the first is defined as the current price EUR/good and the second as the price expected after bought "max_alt_to_receive".

In the case of sell_market is the opposite, so max_price is the current price, and min_price is the expected price after selling "max_alt_to_receive".
It is computed easily because we call get_sell_price passing "max_alt_to_receive".

## Step 3
To continue we need some assumptions:
- The quantity received increasy linearly when we increase the quantity send for the buy_market, and decrease linearly when selling for sell_market.
- max_price can't be lower the min_price (this case is handled skipping the trade)


Let's see starting from analyzing why B is skipped as sell_market:

![](assets/b.png)

If the sell_max_price is lower or equal than buy_min_price can't be profit, so only C and D (theoretically) can be profitable

Now imagine that we have a parameter x (0<=x<=1) which is how much of the "max_eur_to_send" we will actually send.
For example if x is 0.5, then the price after the trade correspond to Average(min_price, max_price) for each market.

In the case of C we see that we shouldn't sell all the "max_eur_to_send" quantity, because after the orange line is no longer profitable, if you don't understand imagine like starting with x=0, and increase it gradually, and try to visualize the 2 actual "prices after the trade" for A and C (note that if x=1, both the rectangles become full filled by orange).

Increasing by x these 2 prices will be equal at some point, the corresponding x, if multiplied by "max_eur_to_send", is the the eur to send to make the maximum profit, then we aproximate by default the quantity requested when sending x*"max_eur_to_send", and use it to compute the profit after calling get_sell_price with that quantity.

![](assets/c.png)

If sell_min_price is greater or equal than buy_max_price we can have a problem with the previous computations, but we are lucky because we can send all the "max_eur_to_send" quantity (Or we can say that we set x as 1).

![](assets/d.png)

## Step 4
Given the market pair, good, quantity of good to request and quantity of eur to send; which can maximize the profit, we will execute that trade.

# Some adaptations
In the case of buy_market max_price is lower than min_price, then max_price=min_price, the opposite in the case of sell_market, if this occours is an exploitation of the market. buy_market and sell_market in a trade could be the same, if this, is another exploitation.
It can be returned also a rest of good along with the EUR good, because in the first trade there can be the possibility that the goods or prices in the sell_market could change, so it tries to sell all goods at the maximum price as possible, and return the rest.