import React, { useEffect, useState } from 'react'
import { Form, Grid } from 'semantic-ui-react'

import { useSubstrate } from './substrate-lib'
import { TxButton } from './substrate-lib/components'

import KittyCards from './KittyCards'

export default function Kitties (props) {
  const { api, keyring } = useSubstrate()
  const { accountPair } = props

  const [kittyCount, setKittyCount] = useState(0)
  const [kitties, setKitties] = useState([])
  const [status, setStatus] = useState('')

  const fetchKitties = () => {


    let unsubscribe
      api.query.kittiesModule.kittiesCount( kittyCount_ => {
      // The storage value is an Option<u32>
      // So we have to check whether it is None first
      // There is also unwrapOr
        let kittyCount = kittyCount_.unwrap().toNumber();
    	console.log("1111111111111111111i22222:"+ kittyCount); 
        setKittyCount(kittyCount);
        
      
      
      }).then(unsub => {
      unsubscribe = unsub
    })
      .catch(console.error)

    return () => unsubscribe && unsubscribe()

  }

  const populateKitties = () => {
    // TODO: 在这里添加额外的逻辑。你需要组成这样的数组结构：
    //  ```javascript
    //  const kitties = [{
    //    id: 0,
    //    dna: ...,
    //    owner: ...
    //  }, { id: ..., dna: ..., owner: ... }]
    //  ```
    // 这个 kitties 会传入 <KittyCards/> 然后对每只猫咪进行处理
    //const kitties = []
    //setKitties(kitties)
    //
    console.log("1111111111111111111");
    if(kittyCount>0){
 //      let kittyCount = kittyCount_.unwrap().toNumber()
       const kittyIndexList = [...Array(kittyCount ).keys()]
       const a = api.query.kittiesModule.kitties.multi(kittyIndexList );
       const b = api.query.kittiesModule.owner.multi(kittyIndexList );
       Promise.all(
           [a,b]
       ).then(([dnaList, ownerlist]) => {
         	
           var arr = new Array();
           dnaList.forEach((item, key, map) =>{
             //const kitty = {id: 0,  dna: item.toU8a(), owner: ownerlist[key].unwrap().toString()};
             const kitty = {id: 0,  dna: item.unwrap(), owner: ownerlist[key].unwrap().toString()};
             arr.push(kitty);

           } );

           setKitties(arr);

       }).catch((error) => {
       })

    }


  }

  useEffect(fetchKitties, [api, keyring])
  useEffect(populateKitties, [kittyCount])

  return <Grid.Column width={16}>
    <h1>小毛孩</h1>
    <KittyCards kitties={kitties} accountPair={accountPair} setStatus={setStatus}/>
    <Form style={{ margin: '1em 0' }}>
      <Form.Field style={{ textAlign: 'center' }}>
        <TxButton
          accountPair={accountPair} label='创建小毛孩' type='SIGNED-TX' setStatus={setStatus}
          attrs={{
            palletRpc: 'kittiesModule',
            callable: 'create',
            inputParams: [],
            paramFields: []
          }}
        />
      </Form.Field>
    </Form>
    <div style={{ overflowWrap: 'break-word' }}>{status}</div>
  </Grid.Column>
}
