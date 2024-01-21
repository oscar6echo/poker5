
# Poker

This is a Texas Hold'em Poker hand evaluator with associated server:

+ [Key Generator](./poker_keygen/)
+ [Hand Evaluator](./poker_eval/)
+ [Server](./poker_server/)
  
It is **very fast**, e.g. on my machine, all 133 million possible 7-card hands ($C^{7}_{52}$) are evaluated in 1.2s.

# Commands

See [justfile](./justfile).

```sh
# test poker crate
cargo test -p poker --lib --release

# build
cargo build --release

# start server
cargo run -p poker_server --release
# ---------- poker server ----------
# 2024-01-21T17:18:37.072607Z  INFO poker_server: build_app_state runtime = 1.216176641s s
# 2024-01-21T17:18:37.072750Z  INFO poker_server: Listening on 127.0.0.1:3000
# 2024-01-21T17:19:27.079487Z  INFO healthz: poker_server: -> OK

# requests

curl http://localhost:3000/healthz
# Ok%

curl http://localhost:3000/config
# {"face":["2","3","4","5","6","7","8","9","T","J","Q","K","A"],"suit":["C","D","H","S"],"card_no":{"2C":0,"QS":43,"5S":15,"9C":28,"8D":25,"5D":13,"JD":37,"QC":40,"2D":1,"7C":20,"TC":32,"6C":16,"9D":29,"3C":4,"5H":14,"4D":9,"7S":23,"8C":24,"QD":41,"KC":44,"KH":46,"6H":18,"3S":7,"QH":42,"4S":11,"TS":35,"KD":45,"AD":49,"AH":50,"TH":34,"2H":2,"7D":21,"6D":17,"9H":30,"AS":51,"TD":33,"5C":12,"8S":27,"9S":31,"4H":10,"7H":22,"3H":6,"JC":36,"JS":39,"6S":19,"AC":48,"JH":38,"8H":26,"4C":8,"2S":3,"KS":47,"3D":5},"card_sy":{"18":"6H","50":"AH","13":"5D","1":"2D","36":"JC","15":"5S","23":"7S","35":"TS","24":"8C","17":"6D","32":"TC","38":"JH","30":"9H","26":"8H","37":"JD","10":"4H","29":"9D","28":"9C","12":"5C","46":"KH","5":"3D","2":"2H","43":"QS","19":"6S","21":"7D","22":"7H","27":"8S","42":"QH","47":"KS","0":"2C","8":"4C","14":"5H","16":"6C","33":"TD","34":"TH","3":"2S","4":"3C","39":"JS","41":"QD","44":"KC","11":"4S","25":"8D","7":"3S","40":"QC","6":"3H","9":"4D","31":"9S","45":"KD","48":"AC","20":"7C","49":"AD","51":"AS"}}%

curl http://localhost:3000/stats-five
# {"straight":{"nb_hand":10,"min_rank":5853,"max_rank":5862,"nb_occur":10200},"full-house":{"nb_hand":156,"min_rank":7140,"max_rank":7295,"nb_occur":3744},"high-card":{"nb_hand":1277,"min_rank":0,"max_rank":1276,"nb_occur":1302540},"four-of-a-kind":{"nb_hand":156,"min_rank":7296,"max_rank":7451,"nb_occur":624},"flush":{"nb_hand":1277,"min_rank":5863,"max_rank":7139,"nb_occur":5108},"straight-flush":{"nb_hand":10,"min_rank":7452,"max_rank":7461,"nb_occur":40},"one-pair":{"nb_hand":2860,"min_rank":1277,"max_rank":4136,"nb_occur":1098240},"three-of-a-kind":{"nb_hand":858,"min_rank":4995,"max_rank":5852,"nb_occur":54912},"two-pairs":{"nb_hand":858,"min_rank":4137,"max_rank":4994,"nb_occur":123552}}%

curl http://localhost:3000/stats-seven
# {"straight":{"nb_hand":10,"min_rank":5853,"max_rank":5862,"nb_occur":6180020},"full-house":{"nb_hand":156,"min_rank":7140,"max_rank":7295,"nb_occur":3473184},"two-pairs":{"nb_hand":763,"min_rank":4140,"max_rank":4994,"nb_occur":31433400},"four-of-a-kind":{"nb_hand":156,"min_rank":7296,"max_rank":7451,"nb_occur":224848},"straight-flush":{"nb_hand":10,"min_rank":7452,"max_rank":7461,"nb_occur":41584},"high-card":{"nb_hand":407,"min_rank":48,"max_rank":1276,"nb_occur":23294460},"flush":{"nb_hand":1277,"min_rank":5863,"max_rank":7139,"nb_occur":4047644},"three-of-a-kind":{"nb_hand":575,"min_rank":5003,"max_rank":5852,"nb_occur":6461620},"one-pair":{"nb_hand":1470,"min_rank":1295,"max_rank":4136,"nb_occur":58627800}}%

curl  -X POST -H "Content-Type: application/json" -d '{"hands":[[8,29,4,11,32],[9,30,5,12,33]]}' http://localhost:3000/rank-five
# [1768,90]%

curl  -X POST -H "Content-Type: application/json" -d '{"hands":[[8,29,4,11,32,18,19],[9,30,5,12,33,19,20]]}' http://localhost:3000/rank-seven
# [4231,5855]%

curl -X POST -H "Content-Type: application/json" -d '{"players":[[8,29], [4,11]],"table":[]}' http://localhost:3000/calc-det
# [{"win":0.6336246367467459,"tie":0.0520307725730945},{"win":0.2623138181070651,"tie":0.0520307725730945}]%

curl  -X POST -H "Content-Type: application/json" -d '{"players":[[8,29,18], [4,11]],"table":[20,21]}' http://localhost:3000/calc-det
# {"message":"Failed to parse the request body as JSON: players[0]: trailing characters at line 1 column 19"}%

curl  -X POST -H "Content-Type: application/json" -d '{"players":[[8,29], [4,11]],"table":[20,21]}' http://localhost:3000/calc-det
# {"message":"invalid nb table cards: 2 - must be among 0, 3, 4 or 5"}%

curl -X POST -H "Content-Type: application/json" -d '{"players":[[8,9],[11],[]],"table":[15,47,23,33],"nb_game":100000}' http://localhost:3000/calc-mc
# {"win":0.16404656186247454,"tie":0.003365134605384215}%

curl -X POST -H "Content-Type: application/json" -d '{"players":[[8,9],[11],[]],"table":[15,47,23,33],"nb_game":100000000}' http://localhost:3000/calc-mc
# {"win":0.16785060671402427,"tie":0.003491515139660606}%

```
