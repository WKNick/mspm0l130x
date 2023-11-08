# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { _reserved0 : [u8 ; 0x1020] , # [doc = "0x1020 - Interrupt index"] pub iidx : IIDX , _reserved1 : [u8 ; 0x04] , # [doc = "0x1028 - Interrupt mask"] pub imask : IMASK , _reserved2 : [u8 ; 0x04] , # [doc = "0x1030 - Raw interrupt status"] pub ris : RIS , _reserved3 : [u8 ; 0x04] , # [doc = "0x1038 - Masked interrupt status"] pub mis : MIS , _reserved4 : [u8 ; 0x04] , # [doc = "0x1040 - Interrupt set"] pub iset : ISET , _reserved5 : [u8 ; 0x04] , # [doc = "0x1048 - Interrupt clear"] pub iclr : ICLR , _reserved6 : [u8 ; 0x94] , # [doc = "0x10e0 - Event Mode"] pub evt_mode : EVT_MODE , _reserved7 : [u8 ; 0x18] , # [doc = "0x10fc - Module Description"] pub desc : DESC , # [doc = "0x1100 - Transmit data register"] pub txd : TXD , # [doc = "0x1104 - Transmit control register"] pub txctl : TXCTL , # [doc = "0x1108 - Receive data register"] pub rxd : RXD , # [doc = "0x110c - Receive control register"] pub rxctl : RXCTL , _reserved12 : [u8 ; 0xf0] , # [doc = "0x1200 - Special enable authorization register"] pub special_auth : SPECIAL_AUTH , _reserved13 : [u8 ; 0x0c] , # [doc = "0x1210 - Application CPU0 authorization register"] pub app_auth : APP_AUTH , } # [doc = "IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iidx::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iidx`]
module"] pub type IIDX = crate :: Reg < iidx :: IIDX_SPEC > ; # [doc = "Interrupt index"] pub mod iidx ; # [doc = "IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imask`]
module"] pub type IMASK = crate :: Reg < imask :: IMASK_SPEC > ; # [doc = "Interrupt mask"] pub mod imask ; # [doc = "RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"] pub type RIS = crate :: Reg < ris :: RIS_SPEC > ; # [doc = "Raw interrupt status"] pub mod ris ; # [doc = "MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`]
module"] pub type MIS = crate :: Reg < mis :: MIS_SPEC > ; # [doc = "Masked interrupt status"] pub mod mis ; # [doc = "ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iset`]
module"] pub type ISET = crate :: Reg < iset :: ISET_SPEC > ; # [doc = "Interrupt set"] pub mod iset ; # [doc = "ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iclr`]
module"] pub type ICLR = crate :: Reg < iclr :: ICLR_SPEC > ; # [doc = "Interrupt clear"] pub mod iclr ; # [doc = "EVT_MODE (r) register accessor: Event Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evt_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_mode`]
module"] pub type EVT_MODE = crate :: Reg < evt_mode :: EVT_MODE_SPEC > ; # [doc = "Event Mode"] pub mod evt_mode ; # [doc = "DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`desc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`]
module"] pub type DESC = crate :: Reg < desc :: DESC_SPEC > ; # [doc = "Module Description"] pub mod desc ; # [doc = "TXD (r) register accessor: Transmit data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txd`]
module"] pub type TXD = crate :: Reg < txd :: TXD_SPEC > ; # [doc = "Transmit data register"] pub mod txd ; # [doc = "TXCTL (r) register accessor: Transmit control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txctl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txctl`]
module"] pub type TXCTL = crate :: Reg < txctl :: TXCTL_SPEC > ; # [doc = "Transmit control register"] pub mod txctl ; # [doc = "RXD (rw) register accessor: Receive data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxd`]
module"] pub type RXD = crate :: Reg < rxd :: RXD_SPEC > ; # [doc = "Receive data register"] pub mod rxd ; # [doc = "RXCTL (rw) register accessor: Receive control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxctl`]
module"] pub type RXCTL = crate :: Reg < rxctl :: RXCTL_SPEC > ; # [doc = "Receive control register"] pub mod rxctl ; # [doc = "SPECIAL_AUTH (r) register accessor: Special enable authorization register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`special_auth::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@special_auth`]
module"] pub type SPECIAL_AUTH = crate :: Reg < special_auth :: SPECIAL_AUTH_SPEC > ; # [doc = "Special enable authorization register"] pub mod special_auth ; # [doc = "APP_AUTH (r) register accessor: Application CPU0 authorization register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_auth::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_auth`]
module"] pub type APP_AUTH = crate :: Reg < app_auth :: APP_AUTH_SPEC > ; # [doc = "Application CPU0 authorization register"] pub mod app_auth ;