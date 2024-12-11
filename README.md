最初の写経実装はlazy_staticを使っていて、これだとランダムにオブジェクトを配置出来ない  
なのでVec<Sphere>のStructを作り、トレイトでintersectionを実装  
またオプションで  
  s sampling数 w:横幅 m:モデル番号 を指定可能に  
  
