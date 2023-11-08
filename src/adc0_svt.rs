# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { _reserved0 : [u8 ; 0x0160] , # [doc = "0x160 - FIFO Data Register"] pub fifodata : FIFODATA , _reserved1 : [u8 ; 0x011c] , # [doc = "0x280..0x290 - Memory Result Register"] pub memres : [MEMRES ; 4] , } # [doc = "FIFODATA (r) register accessor: FIFO Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifodata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifodata`]
module"] pub type FIFODATA = crate :: Reg < fifodata :: FIFODATA_SPEC > ; # [doc = "FIFO Data Register"] pub mod fifodata ; # [doc = "MEMRES (r) register accessor: Memory Result Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memres::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memres`]
module"] pub type MEMRES = crate :: Reg < memres :: MEMRES_SPEC > ; # [doc = "Memory Result Register"] pub mod memres ;