import React, { useEffect, useState } from 'react'
import { Form, Grid } from 'semantic-ui-react'

import { useSubstrate } from './substrate-lib'
import { TxButton } from './substrate-lib/components'

import KittyCards from './KittyCards'

export default function Kitties (props) {
  const { api, keyring } = useSubstrate()
  const { accountPair } = props

  const [kitties, setKitties] = useState([])
  const [status, setStatus] = useState('')

  // console.log(accountPair)

  // let data = Array()
  const [skittyIndex, setkittyIndex] = useState(0)
  const [skittiesList, setkittiesList] = useState([])
  const [sownersList, setownersList] = useState([])

  const fetchKitties = () => {
    // TODO: 在这里调用 `api.query.kittiesModule.*` 函数去取得猫咪的信息。
    // 你需要取得：
    //   - 共有多少只猫咪
    //   - 每只猫咪的主人是谁
    //   - 每只猫咪的 DNA 是什么，用来组合出它的形态
    console.log(`fetchKitties start`)
    let kittyIndex
    let kittiesList = Array()
    let ownersList = Array()
    // TODO: 在这里调用 `api.query.kittiesModule.*` 函数去取得猫咪的信息。
    // 你需要取得：
    //   - 共有多少只猫咪
    //   - 每只猫咪的主人是谁
    //   - 每只猫咪的 DNA 是什么，用来组合出它的形态
    // console.log(api)
    // let kittyIndex
    // let kittiesList = Array()
    // let ownersList = Array()
    api.query.kittiesModule.kittiesCount(d => {

      (async () => {
        let owner = await api.query.kittiesModule.owner(0);
        console.log("<<<<<owner1>>>>>>: " + owner.isSome);
     })();


      // console.log(d)
      //let kittyIndex
      if (d.isSome) {
        kittyIndex = d.unwrap().toNumber()
        // console.log(kittyIndex)
      }
      
      //console.log('kittyIndex: ' + kittyIndex)
      setkittyIndex(kittyIndex)
      // let kittiesList = Array()
      // let ownersList = Array()
      // let ttt
      // api.query.kittiesModule.kitties(kittyIndex - 1).then(v => {
      //   if (v.isSome) {
      //     ttt = v.unwrap().toU8a()
      //   }
      //   console.log(ttt[0])
      // })
      // let owner
      // api.query.kittiesModule.owner(kittyIndex - 1).then(v => {
      //   if (v.isSome) {
      //     owner = v.unwrap().toString()
      //   }
      //   console.log(owner)
      // })
      const kittyIndexList = createNArray(kittyIndex)
      console.log('kittyIndexList is:' + kittyIndexList)
      api.query.kittiesModule.kitties.multi(kittyIndexList, (_kitties) => {
        // console.log(_kitties)
        for (let i in _kitties) {
          if (_kitties[i].isSome) {
            const _kitty = _kitties[i].unwrap().toU8a()
            console.log(`${i} = ${_kitty}`)
            kittiesList.push(_kitty)
          }
        }
        //console.log('kittiesList: ' + kittiesList.join("|"))
        setkittiesList(kittiesList)
      })
      // api.query.kittiesModule.owner.multi(kittyIndexList, (owners) => {
      //   // console.log(owners)
      //   // console.log(typeof(owners))
      //   for (let i in owners) {
      //     let owner = owners[i]
      //     if (owners[i].isSome) {
      //       const _owner = owners[i].unwrap().toString()
      //       console.log(`_owner is ${i} = ${_owner}`)
      //       ownersList.push(_owner)
      //     }
      //   }
      //   //console.log('ownersList: ' + ownersList.join("|"))
      //   setownersList(ownersList)
      // })

     

      console.log("kittyIndexList>>>>>: " + kittyIndexList);
      api.query.kittiesModule.owner.multi(kittyIndexList, (owners) => {
        // console.log(owners)
        // console.log(typeof(owners))
        for (let i in owners) {
          let owner = owners[i]
          console.log("owner>>>>>>: " + owner.isSome);
          if (owner.isSome) {
            const _owner = owner.unwrap().toString()
            console.log(`_owner is   = ${_owner}`)
          
          }
        }
       
      })

    })

    console.log(`fetchKitties end`)
  }

  const populateKitties = () => {
    (async () => {
      let owner = await api.query.kittiesModule.owner(0);
      console.log("<<<<<owner2>>>>>>: " + owner.isSome);
   })();

    // TODO: 在这里添加额外的逻辑。你需要组成这样的数组结构：
    //  ```javascript
    //  const kitties = [{
    //    id: 0,
    //    dna: ...,
    //    owner: ...
    //  }, { id: ..., dna: ..., owner: ... }]
    //  ```
    // 这个 kitties 会传入 <KittyCards/> 然后对每只猫咪进行处理
    console.log(`populateKitties start`)
    let kitties = Array()
    // accountPair.address有地址信息

    // 只显示自己的猫
    for(let i = 0; i<skittyIndex; i++) { 
      kitties.push({
        id: i,
        dna: skittiesList[i],
        owner: sownersList[i]
      })
    }
    // console.log('skittyIndex: ' + skittyIndex)
    // console.log('skittiesList: ' + skittiesList.join("|"))
    // console.log('sownersList: ' + sownersList.join("|"))
    console.log(kitties)
    setKitties(kitties)
    console.log(`populateKitties end`)
  }

  // useEffect(fetchKitties, [api, keyring])
  // useEffect(populateKitties, [])

  useEffect(fetchKitties, [api, keyring, api.query.kittiesModule])
  // useEffect(fetchKitties1, [api, keyring])
  useEffect(populateKitties, [skittyIndex, skittiesList, sownersList])
  //useEffect(populateKitties)

  return <Grid.Column width={16}>
    <h1>小毛孩</h1>
    <KittyCards kitties={kitties} accountPair={accountPair} setStatus={setStatus}/>
    <Form style={{ margin: '1em 0' }}>
      <Form.Field style={{ textAlign: 'center' }}>
        <TxButton
          accountPair={accountPair} label='创建小毛孩' type='SIGNED-TX' setStatus={setStatus}
          attrs={{
            palletRpc: 'kittiesModule',
            callable: 'createKitty',
            inputParams: [],
            paramFields: []
          }}
        />
      </Form.Field>
    </Form>
    <div style={{ overflowWrap: 'break-word' }}>{status}</div>
  </Grid.Column>
}


function createNArray(n) {
  let arr = [];
  for (let i = 0; i < n; i++) {
      arr.push(i)
  }
  return arr;
}