pub struct VirtualAsset {
    pub id: String,
    pub owner: String,
}

pub struct Metaverse {
    pub assets: Vec<VirtualAsset>,
}

impl Metaverse {
    pub fn new() -> Self {
        Self { assets: vec![] }
    }

    pub fn create_asset(&mut self, asset: VirtualAsset) {
        self.assets.push(asset);
    }
}
