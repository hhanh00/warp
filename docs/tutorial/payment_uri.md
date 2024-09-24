# Using Payment URIs

Instead of using a JSON object,

```
zcash-warp〉make-payment-uri '{"recipients":[{"address":"uregtest1chggrm62vhkf5ftv3mawjvavuwjgx7x3wj582tlq5769cl6xf6q6whrukc5z2mxyz603j6lzda8pgyy8dy59qta58uncp6cetwmweklawr2ty34jqw6e8d704rly7m7eh9wcs50m0hdvsdg5akzys3uh95wugw9n78a5gj0mc0m69rsu2lf32tl8glje9p6870e9wwy6qeeyvg6x7ae","amount":100000000,"pools":1,"memo":null,"memo_bytes":null},{"address":"uregtest1chggrm62vhkf5ftv3mawjvavuwjgx7x3wj582tlq5769cl6xf6q6whrukc5z2mxyz603j6lzda8pgyy8dy59qta58uncp6cetwmweklawr2ty34jqw6e8d704rly7m7eh9wcs50m0hdvsdg5akzys3uh95wugw9n78a5gj0mc0m69rsu2lf32tl8glje9p6870e9wwy6qeeyvg6x7ae","amount":100000000,"pools":1,"memo":null,"memo_bytes":null},{"address":"uregtest1chggrm62vhkf5ftv3mawjvavuwjgx7x3wj582tlq5769cl6xf6q6whrukc5z2mxyz603j6lzda8pgyy8dy59qta58uncp6cetwmweklawr2ty34jqw6e8d704rly7m7eh9wcs50m0hdvsdg5akzys3uh95wugw9n78a5gj0mc0m69rsu2lf32tl8glje9p6870e9wwy6qeeyvg6x7ae","amount":100000000,"pools":2,"memo":null,"memo_bytes":null},{"address":"uregtest1chggrm62vhkf5ftv3mawjvavuwjgx7x3wj582tlq5769cl6xf6q6whrukc5z2mxyz603j6lzda8pgyy8dy59qta58uncp6cetwmweklawr2ty34jqw6e8d704rly7m7eh9wcs50m0hdvsdg5akzys3uh95wugw9n78a5gj0mc0m69rsu2lf32tl8glje9p6870e9wwy6qeeyvg6x7ae","amount":100000000,"pools":2,"memo":null,"memo_bytes":null},{"address":"uregtest1chggrm62vhkf5ftv3mawjvavuwjgx7x3wj582tlq5769cl6xf6q6whrukc5z2mxyz603j6lzda8pgyy8dy59qta58uncp6cetwmweklawr2ty34jqw6e8d704rly7m7eh9wcs50m0hdvsdg5akzys3uh95wugw9n78a5gj0mc0m69rsu2lf32tl8glje9p6870e9wwy6qeeyvg6x7ae","amount":100000000,"pools":2,"memo":null,"memo_bytes":null},{"address":"uregtest1chggrm62vhkf5ftv3mawjvavuwjgx7x3wj582tlq5769cl6xf6q6whrukc5z2mxyz603j6lzda8pgyy8dy59qta58uncp6cetwmweklawr2ty34jqw6e8d704rly7m7eh9wcs50m0hdvsdg5akzys3uh95wugw9n78a5gj0mc0m69rsu2lf32tl8glje9p6870e9wwy6qeeyvg6x7ae","amount":100000000,"pools":4,"memo":null,"memo_bytes":null}],"src_pools":7,"sender_pay_fees":true,"use_change":true,"height":102,"expiration":202}'
```

One can create a Payment URI.

```
zcash:?address=tmFtne18VWwk7x8p2wUwfnAqtntMVikctYi&amount=1&address.1=tmFtne18VWwk7x8p2wUwfnAqtntMVikctYi&amount.1=1&address.2=zregtestsapling1h0ud99tf3qe9lmpmncs8rz0vwluhav0etuuejmvxcjnykd6gvp8tllh77ac0dm60zqmyq3c2a3t&amount.2=1&memo.2=9g&address.3=zregtestsapling1h0ud99tf3qe9lmpmncs8rz0vwluhav0etuuejmvxcjnykd6gvp8tllh77ac0dm60zqmyq3c2a3t&amount.3=1&memo.3=9g&address.4=zregtestsapling1h0ud99tf3qe9lmpmncs8rz0vwluhav0etuuejmvxcjnykd6gvp8tllh77ac0dm60zqmyq3c2a3t&amount.4=1&memo.4=9g&address.5=uregtest1qae0mnr0p9jenge23nc47f359kwkyqvq0kl3qfgyqmk5k37jcvqlzafype9r99yxwfhs38q3w0a2qgam489deht2fguv6hg6g5m0c5vk&amount.5=1&memo.5=9g
```

The URI is much shorter than the JSON object, but some information will be lost (sender pays fees, use change).

::: tip
Yet, they can be a good option for storing frequently used payments, and they support multiple recipients too.
:::

Then use them to pay
```
zcash-warp〉pay-payment-uri 1 'zcash:?address=tmFtne18VWwk7x8p2wUwfnAqtntMVikctYi&amount=1&address.1=tmFtne18VWwk7x8p2wUwfnAqtntMVikctYi&amount.1=1&address.2=zregtestsapling1h0ud99tf3qe9lmpmncs8rz0vwluhav0etuuejmvxcjnykd6gvp8tllh77ac0dm60zqmyq3c2a3t&amount.2=1&memo.2=9g&address.3=zregtestsapling1h0ud99tf3qe9lmpmncs8rz0vwluhav0etuuejmvxcjnykd6gvp8tllh77ac0dm60zqmyq3c2a3t&amount.3=1&memo.3=9g&address.4=zregtestsapling1h0ud99tf3qe9lmpmncs8rz0vwluhav0etuuejmvxcjnykd6gvp8tllh77ac0dm60zqmyq3c2a3t&amount.4=1&memo.4=9g&address.5=uregtest1qae0mnr0p9jenge23nc47f359kwkyqvq0kl3qfgyqmk5k37jcvqlzafype9r99yxwfhs38q3w0a2qgam489deht2fguv6hg6g5m0c5vk&amount.5=1&memo.5=9g'
```

Next, we'll consider the scenario where you want to transfer the entire balance
of your account to another address.

