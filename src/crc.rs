# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { _reserved0 : [u8 ; 0x0800] , # [doc = "0x800 - Power enable"] pub pwren : PWREN , # [doc = "0x804 - Reset Control"] pub rstctl : RSTCTL , _reserved2 : [u8 ; 0x0c] , # [doc = "0x814 - Status Register"] pub stat : STAT , _reserved3 : [u8 ; 0x08e4] , # [doc = "0x10fc - Module Description"] pub desc : DESC , # [doc = "0x1100 - CRC Control Register"] pub crcctrl : CRCCTRL , # [doc = "0x1104 - CRC Seed Register"] pub crcseed : CRCSEED , # [doc = "0x1108 - CRC Input Data Register"] pub crcin : CRCIN , # [doc = "0x110c - CRC Output Result Register"] pub crcout : CRCOUT , _reserved8 : [u8 ; 0x06f0] , # [doc = "0x1800..0x2000 - CRC Input Data Array Register"] pub crcin_idx : [CRCIN_IDX ; 512] , } # [doc = "PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwren::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwren::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwren`]
module"] pub type PWREN = crate :: Reg < pwren :: PWREN_SPEC > ; # [doc = "Power enable"] pub mod pwren ; # [doc = "RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctl`]
module"] pub type RSTCTL = crate :: Reg < rstctl :: RSTCTL_SPEC > ; # [doc = "Reset Control"] pub mod rstctl ; # [doc = "STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"] pub type STAT = crate :: Reg < stat :: STAT_SPEC > ; # [doc = "Status Register"] pub mod stat ; # [doc = "DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`desc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`]
module"] pub type DESC = crate :: Reg < desc :: DESC_SPEC > ; # [doc = "Module Description"] pub mod desc ; # [doc = "CRCCTRL (rw) register accessor: CRC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcctrl`]
module"] pub type CRCCTRL = crate :: Reg < crcctrl :: CRCCTRL_SPEC > ; # [doc = "CRC Control Register"] pub mod crcctrl ; # [doc = "CRCSEED (w) register accessor: CRC Seed Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcseed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcseed`]
module"] pub type CRCSEED = crate :: Reg < crcseed :: CRCSEED_SPEC > ; # [doc = "CRC Seed Register"] pub mod crcseed ; # [doc = "CRCIN (w) register accessor: CRC Input Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcin::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcin`]
module"] pub type CRCIN = crate :: Reg < crcin :: CRCIN_SPEC > ; # [doc = "CRC Input Data Register"] pub mod crcin ; # [doc = "CRCOUT (r) register accessor: CRC Output Result Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcout::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcout`]
module"] pub type CRCOUT = crate :: Reg < crcout :: CRCOUT_SPEC > ; # [doc = "CRC Output Result Register"] pub mod crcout ; # [doc = "CRCIN_IDX (w) register accessor: CRC Input Data Array Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcin_idx::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcin_idx`]
module"] pub type CRCIN_IDX = crate :: Reg < crcin_idx :: CRCIN_IDX_SPEC > ; # [doc = "CRC Input Data Array Register"] pub mod crcin_idx ;