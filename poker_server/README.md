# Poker server

Designed to expose crate [poker_eval](https://github.com/oscar6echo/poker5/tree/main/poker_eval) as an API.  

## Run

```sh
# build
cargo build --release

# run
./target/release/poker_server --addr 127.0.0.1 --port 3000

---------- poker server ----------
2024-02-04T21:27:00.653170Z  INFO poker_server: build_app_state runtime = 1.091433858s s
2024-02-04T21:27:00.653309Z  INFO poker_server: Listening on 127.0.0.1:3000

```

Or:

```rust
use poker_server::main;
main();
```

## Use

```sh
# healthz
curl http://localhost:3000/healthz
# Ok%

# config
curl http://localhost:3000/config
# {"face":["2","3","4","5","6","7","8","9","T","J","Q","K","A"],"suit":["C","D","H","S"],"card_no":{"6D":17,"9C":28,"5C":12,"6C":16,"9S":31,"QS":43,"QH":42,"KH":46,"2S":3,"QD":41,"7S":23,"9H":30,"JD":37,"5S":15,"KD":45,"TC":32,"5H":14,"AH":50,"AD":49,"3H":6,"7H":22,"9D":29,"KC":44,"4D":9,"6H":18,"4C":8,"TD":33,"8C":24,"JH":38,"JS":39,"AS":51,"8S":27,"TH":34,"6S":19,"5D":13,"2D":1,"3S":7,"3D":5,"4S":11,"4H":10,"7D":21,"8D":25,"JC":36,"2H":2,"8H":26,"KS":47,"TS":35,"AC":48,"QC":40,"2C":0,"7C":20,"3C":4},"card_sy":{"17":"6D","38":"JH","48":"AC","22":"7H","2":"2H","42":"QH","32":"TC","8":"4C","0":"2C","14":"5H","39":"JS","18":"6H","11":"4S","3":"2S","45":"KD","47":"KS","13":"5D","20":"7C","24":"8C","51":"AS","21":"7D","44":"KC","41":"QD","40":"QC","46":"KH","33":"TD","9":"4D","19":"6S","26":"8H","30":"9H","23":"7S","36":"JC","49":"AD","37":"JD","50":"AH","12":"5C","7":"3S","15":"5S","25":"8D","27":"8S","5":"3D","31":"9S","35":"TS","34":"TH","10":"4H","43":"QS","4":"3C","16":"6C","1":"2D","28":"9C","6":"3H","29":"9D"}}%

# stats for 5-card hands
curl http://localhost:3000/stats-five
# {"four-of-a-kind":{"nb_hand":156,"min_rank":7296,"max_rank":7451,"nb_occur":624},"full-house":{"nb_hand":156,"min_rank":7140,"max_rank":7295,"nb_occur":3744},"flush":{"nb_hand":1277,"min_rank":5863,"max_rank":7139,"nb_occur":5108},"one-pair":{"nb_hand":2860,"min_rank":1277,"max_rank":4136,"nb_occur":1098240},"straight":{"nb_hand":10,"min_rank":5853,"max_rank":5862,"nb_occur":10200},"high-card":{"nb_hand":1277,"min_rank":0,"max_rank":1276,"nb_occur":1302540},"two-pairs":{"nb_hand":858,"min_rank":4137,"max_rank":4994,"nb_occur":123552},"straight-flush":{"nb_hand":10,"min_rank":7452,"max_rank":7461,"nb_occur":40},"three-of-a-kind":{"nb_hand":858,"min_rank":4995,"max_rank":5852,"nb_occur":54912}}%

# stats for 7-card hands
curl http://localhost:3000/stats-seven
# {"straight":{"nb_hand":10,"min_rank":5853,"max_rank":5862,"nb_occur":6180020},"high-card":{"nb_hand":407,"min_rank":48,"max_rank":1276,"nb_occur":23294460},"four-of-a-kind":{"nb_hand":156,"min_rank":7296,"max_rank":7451,"nb_occur":224848},"straight-flush":{"nb_hand":10,"min_rank":7452,"max_rank":7461,"nb_occur":41584},"two-pairs":{"nb_hand":763,"min_rank":4140,"max_rank":4994,"nb_occur":31433400},"full-house":{"nb_hand":156,"min_rank":7140,"max_rank":7295,"nb_occur":3473184},"one-pair":{"nb_hand":1470,"min_rank":1295,"max_rank":4136,"nb_occur":58627800},"flush":{"nb_hand":1277,"min_rank":5863,"max_rank":7139,"nb_occur":4047644},"three-of-a-kind":{"nb_hand":575,"min_rank":5003,"max_rank":5852,"nb_occur":6461620}}%

# get 5-card hand rank
curl  -X POST -H "Content-Type: application/json" -d '{"hands":[[8,29,4,11,32],[9,30,5,12,33]]}' http://localhost:3000/rank-five
# [1768,90]%

# get 7-card hand rank
curl  -X POST -H "Content-Type: application/json" -d '{"hands":[[8,29,4,11,32,18,19],[9,30,5,12,33,19,20]]}' http://localhost:3000/rank-seven
# [4231,5855]%

# calc equity - determenistic mode
curl -X POST -H "Content-Type: application/json" -d '{"players":[[8,29], [4,11]],"table":[]}' http://localhost:3000/calc-det
# [{"win":0.6336246367467459,"tie":0.0520307725730945},{"win":0.2623138181070651,"tie":0.0520307725730945}]%

# calc equity - monte carlo mode
time curl -X POST -H "Content-Type: application/json" -d '{"players":[[8,9],[11],[]],"table":[15,47,23,33],"nb_game":100000000}' http://localhost:3000/calc-mc
# {"win":0.1676650867066035,"tie":0.003498295139931806}
# 0.00s user 0.01s system 0% cpu 0.939 total
# 300m hands ranks in <1s - quite fast!
```
