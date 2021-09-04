const { ApiPromise, WsProvider } = require('@polkadot/api');

test();

async function test() {
    // Construct
    const wsProvider = new WsProvider('ws://127.0.0.1:9944');
    // 如没有运行 node-template，也可试连到波卡主网上： `wss://rpc.polkadot.io`.
    const api = await ApiPromise.create({
        provider: wsProvider,
        types: {
            KittyIndex: 'u64',
            Kitty: {
                value: '[u8; 16]'
            },
        }
    });

    const data = await api.rpc.state.getMetadata();
    const kittyCount = `${await api.query.kittiesModule.kittiesCount()}`;
    console.log("kittyCount: " + kittyCount);
    let kittyIds = createNArray(kittyCount);

    // let kitties = kittyIds.map(kittyId => {
    //     const kitty = await api.query.kittiesModule.kitties(kittyId);
    //     let owner = await api.query.kittiesModule.owner(kittyId);
    //     kitty.owner = owner
    //     return kitty
    // })

    // for(let i=0; i<kittyIds.length; i++){
    //     let kittyId = i;
    //             const kitty = await api.query.kittiesModule.kitties(kittyId);
    //     let owner = await api.query.kittiesModule.owner(kittyId);
    //  //   kitty.owner = owner
    //    // let k= kitty.unwrap();
    //     console.log("kittyid: " + kittyId);
    //     console.log("kitty some: " + kitty.isSome);
    //     if( kitty.isSome) {
    //         let k= kitty.unwrap();
    //         console.log("kitty: " + k);
    //     }
    // }

    console.log("kittyIds>>>>>>>>>>>: " + kittyIds);
    api.query.kittiesModule.owner.multi(kittyIds, (owners) => {
        // console.log(owners)
        // console.log(typeof(owners))
        for (let i in owners) {
          let owner = owners[i]
          console.log("owner: " + i);
          if (owners[i].isSome) {
            const _owner = owners[i].unwrap().toString()
            console.log(`_owner is ${i} = ${_owner}`)
          
          }
        }
       
      })
    
      let owner = await api.query.kittiesModule.owner(0);
      console.log("*******>>>>>>: " + owner.isSome);
      console.log("*******>>>>>>: " + owner.unwrap());
   // console.log("kitty: " + kitties);
}

function createNArray(n) {
    let arr = [];
    for (let i = 0; i < n; i++) {
        arr.push(i)
    }
    return arr;
}