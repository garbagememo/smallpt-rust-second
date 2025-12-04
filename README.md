最初の写経実装はlazy_staticを使っていて、これだとランダムにオブジェクトを配置出来ない  
なのでVec<Sphere>のStructを作り、トレイトでintersectionを実装  
またオプションで  
  s sampling数 w:横幅 m:モデル番号 を指定可能に  

  BVHを導入する場合、Box<dyn Shape>等とするとメタクソ遅くなるので一旦元に戻して調査を続行する
