# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { _reserved0 : [u8 ; 0x0800] , # [doc = "0x800 - Power enable"] pub pwren : PWREN , # [doc = "0x804 - Reset Control"] pub rstctl : RSTCTL , # [doc = "0x808 - Peripheral Clock Configuration Register"] pub clkcfg : CLKCFG , _reserved3 : [u8 ; 0x08] , # [doc = "0x814 - Status Register"] pub gprcm_stat : GPRCM_STAT , _reserved4 : [u8 ; 0x07e8] , # [doc = "0x1000 - Clock Divider"] pub clkdiv : CLKDIV , _reserved5 : [u8 ; 0x04] , # [doc = "0x1008 - Clock Select for Ultra Low Power peripherals"] pub clksel : CLKSEL , _reserved6 : [u8 ; 0x0c] , # [doc = "0x1018 - Peripheral Debug Control"] pub pdbgctl : PDBGCTL , _reserved7 : [u8 ; 0x04] , # [doc = "0x1020 - Interrupt index"] pub int_event0_iidx : INT_EVENT0_IIDX , _reserved8 : [u8 ; 0x04] , # [doc = "0x1028 - Interrupt mask"] pub int_event0_imask : INT_EVENT0_IMASK , _reserved9 : [u8 ; 0x04] , # [doc = "0x1030 - Raw interrupt status"] pub int_event0_ris : INT_EVENT0_RIS , _reserved10 : [u8 ; 0x04] , # [doc = "0x1038 - Masked interrupt status"] pub int_event0_mis : INT_EVENT0_MIS , _reserved11 : [u8 ; 0x04] , # [doc = "0x1040 - Interrupt set"] pub int_event0_iset : INT_EVENT0_ISET , _reserved12 : [u8 ; 0x04] , # [doc = "0x1048 - Interrupt clear"] pub int_event0_iclr : INT_EVENT0_ICLR , _reserved13 : [u8 ; 0x04] , # [doc = "0x1050 - Interrupt index"] pub int_event1_iidx : INT_EVENT1_IIDX , _reserved14 : [u8 ; 0x04] , # [doc = "0x1058 - Interrupt mask"] pub int_event1_imask : INT_EVENT1_IMASK , _reserved15 : [u8 ; 0x04] , # [doc = "0x1060 - Raw interrupt status"] pub int_event1_ris : INT_EVENT1_RIS , _reserved16 : [u8 ; 0x04] , # [doc = "0x1068 - Masked interrupt status"] pub int_event1_mis : INT_EVENT1_MIS , _reserved17 : [u8 ; 0x04] , # [doc = "0x1070 - Interrupt set"] pub int_event1_iset : INT_EVENT1_ISET , _reserved18 : [u8 ; 0x04] , # [doc = "0x1078 - Interrupt clear"] pub int_event1_iclr : INT_EVENT1_ICLR , _reserved19 : [u8 ; 0x04] , # [doc = "0x1080 - Interrupt index"] pub int_event2_iidx : INT_EVENT2_IIDX , _reserved20 : [u8 ; 0x04] , # [doc = "0x1088 - Interrupt mask"] pub int_event2_imask : INT_EVENT2_IMASK , _reserved21 : [u8 ; 0x04] , # [doc = "0x1090 - Raw interrupt status"] pub int_event2_ris : INT_EVENT2_RIS , _reserved22 : [u8 ; 0x04] , # [doc = "0x1098 - Masked interrupt status"] pub int_event2_mis : INT_EVENT2_MIS , _reserved23 : [u8 ; 0x04] , # [doc = "0x10a0 - Interrupt set"] pub int_event2_iset : INT_EVENT2_ISET , _reserved24 : [u8 ; 0x04] , # [doc = "0x10a8 - Interrupt clear"] pub int_event2_iclr : INT_EVENT2_ICLR , _reserved25 : [u8 ; 0x34] , # [doc = "0x10e0 - Event Mode"] pub evt_mode : EVT_MODE , _reserved26 : [u8 ; 0x18] , # [doc = "0x10fc - Module Description"] pub desc : DESC , # [doc = "0x1100 - UART Control Register 0"] pub ctl0 : CTL0 , # [doc = "0x1104 - UART Line Control Register"] pub lcrh : LCRH , # [doc = "0x1108 - UART Status Register"] pub stat : STAT , # [doc = "0x110c - UART Interrupt FIFO Level Select Register"] pub ifls : IFLS , # [doc = "0x1110 - UART Integer Baud-Rate Divisor Register"] pub ibrd : IBRD , # [doc = "0x1114 - UART Fractional Baud-Rate Divisor Register"] pub fbrd : FBRD , # [doc = "0x1118 - Glitch Filter Control"] pub gfctl : GFCTL , _reserved34 : [u8 ; 0x04] , # [doc = "0x1120 - UART Transmit Data Register"] pub txdata : TXDATA , # [doc = "0x1124 - UART Receive Data Register"] pub rxdata : RXDATA , _reserved36 : [u8 ; 0x20] , # [doc = "0x1148 - Self Address Mask Register"] pub amask : AMASK , # [doc = "0x114c - Self Address Register"] pub addr : ADDR , } # [doc = "PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwren::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwren::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwren`]
module"] pub type PWREN = crate :: Reg < pwren :: PWREN_SPEC > ; # [doc = "Power enable"] pub mod pwren ; # [doc = "RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctl`]
module"] pub type RSTCTL = crate :: Reg < rstctl :: RSTCTL_SPEC > ; # [doc = "Reset Control"] pub mod rstctl ; # [doc = "CLKCFG (rw) register accessor: Peripheral Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkcfg`]
module"] pub type CLKCFG = crate :: Reg < clkcfg :: CLKCFG_SPEC > ; # [doc = "Peripheral Clock Configuration Register"] pub mod clkcfg ; # [doc = "GPRCM_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gprcm_stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gprcm_stat`]
module"] pub type GPRCM_STAT = crate :: Reg < gprcm_stat :: GPRCM_STAT_SPEC > ; # [doc = "Status Register"] pub mod gprcm_stat ; # [doc = "CLKDIV (rw) register accessor: Clock Divider\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`]
module"] pub type CLKDIV = crate :: Reg < clkdiv :: CLKDIV_SPEC > ; # [doc = "Clock Divider"] pub mod clkdiv ; # [doc = "CLKSEL (rw) register accessor: Clock Select for Ultra Low Power peripherals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel`]
module"] pub type CLKSEL = crate :: Reg < clksel :: CLKSEL_SPEC > ; # [doc = "Clock Select for Ultra Low Power peripherals"] pub mod clksel ; # [doc = "PDBGCTL (rw) register accessor: Peripheral Debug Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdbgctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdbgctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdbgctl`]
module"] pub type PDBGCTL = crate :: Reg < pdbgctl :: PDBGCTL_SPEC > ; # [doc = "Peripheral Debug Control"] pub mod pdbgctl ; # [doc = "INT_EVENT0_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_event0_iidx::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event0_iidx`]
module"] pub type INT_EVENT0_IIDX = crate :: Reg < int_event0_iidx :: INT_EVENT0_IIDX_SPEC > ; # [doc = "Interrupt index"] pub mod int_event0_iidx ; # [doc = "INT_EVENT0_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_event0_imask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_event0_imask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event0_imask`]
module"] pub type INT_EVENT0_IMASK = crate :: Reg < int_event0_imask :: INT_EVENT0_IMASK_SPEC > ; # [doc = "Interrupt mask"] pub mod int_event0_imask ; # [doc = "INT_EVENT0_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_event0_ris::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event0_ris`]
module"] pub type INT_EVENT0_RIS = crate :: Reg < int_event0_ris :: INT_EVENT0_RIS_SPEC > ; # [doc = "Raw interrupt status"] pub mod int_event0_ris ; # [doc = "INT_EVENT0_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_event0_mis::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event0_mis`]
module"] pub type INT_EVENT0_MIS = crate :: Reg < int_event0_mis :: INT_EVENT0_MIS_SPEC > ; # [doc = "Masked interrupt status"] pub mod int_event0_mis ; # [doc = "INT_EVENT0_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_event0_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event0_iset`]
module"] pub type INT_EVENT0_ISET = crate :: Reg < int_event0_iset :: INT_EVENT0_ISET_SPEC > ; # [doc = "Interrupt set"] pub mod int_event0_iset ; # [doc = "INT_EVENT0_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_event0_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event0_iclr`]
module"] pub type INT_EVENT0_ICLR = crate :: Reg < int_event0_iclr :: INT_EVENT0_ICLR_SPEC > ; # [doc = "Interrupt clear"] pub mod int_event0_iclr ; # [doc = "INT_EVENT1_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_event1_iidx::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event1_iidx`]
module"] pub type INT_EVENT1_IIDX = crate :: Reg < int_event1_iidx :: INT_EVENT1_IIDX_SPEC > ; # [doc = "Interrupt index"] pub mod int_event1_iidx ; # [doc = "INT_EVENT1_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_event1_imask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_event1_imask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event1_imask`]
module"] pub type INT_EVENT1_IMASK = crate :: Reg < int_event1_imask :: INT_EVENT1_IMASK_SPEC > ; # [doc = "Interrupt mask"] pub mod int_event1_imask ; # [doc = "INT_EVENT1_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_event1_ris::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event1_ris`]
module"] pub type INT_EVENT1_RIS = crate :: Reg < int_event1_ris :: INT_EVENT1_RIS_SPEC > ; # [doc = "Raw interrupt status"] pub mod int_event1_ris ; # [doc = "INT_EVENT1_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_event1_mis::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event1_mis`]
module"] pub type INT_EVENT1_MIS = crate :: Reg < int_event1_mis :: INT_EVENT1_MIS_SPEC > ; # [doc = "Masked interrupt status"] pub mod int_event1_mis ; # [doc = "INT_EVENT1_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_event1_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event1_iset`]
module"] pub type INT_EVENT1_ISET = crate :: Reg < int_event1_iset :: INT_EVENT1_ISET_SPEC > ; # [doc = "Interrupt set"] pub mod int_event1_iset ; # [doc = "INT_EVENT1_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_event1_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event1_iclr`]
module"] pub type INT_EVENT1_ICLR = crate :: Reg < int_event1_iclr :: INT_EVENT1_ICLR_SPEC > ; # [doc = "Interrupt clear"] pub mod int_event1_iclr ; # [doc = "INT_EVENT2_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_event2_iidx::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event2_iidx`]
module"] pub type INT_EVENT2_IIDX = crate :: Reg < int_event2_iidx :: INT_EVENT2_IIDX_SPEC > ; # [doc = "Interrupt index"] pub mod int_event2_iidx ; # [doc = "INT_EVENT2_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_event2_imask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_event2_imask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event2_imask`]
module"] pub type INT_EVENT2_IMASK = crate :: Reg < int_event2_imask :: INT_EVENT2_IMASK_SPEC > ; # [doc = "Interrupt mask"] pub mod int_event2_imask ; # [doc = "INT_EVENT2_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_event2_ris::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event2_ris`]
module"] pub type INT_EVENT2_RIS = crate :: Reg < int_event2_ris :: INT_EVENT2_RIS_SPEC > ; # [doc = "Raw interrupt status"] pub mod int_event2_ris ; # [doc = "INT_EVENT2_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_event2_mis::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event2_mis`]
module"] pub type INT_EVENT2_MIS = crate :: Reg < int_event2_mis :: INT_EVENT2_MIS_SPEC > ; # [doc = "Masked interrupt status"] pub mod int_event2_mis ; # [doc = "INT_EVENT2_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_event2_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event2_iset`]
module"] pub type INT_EVENT2_ISET = crate :: Reg < int_event2_iset :: INT_EVENT2_ISET_SPEC > ; # [doc = "Interrupt set"] pub mod int_event2_iset ; # [doc = "INT_EVENT2_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_event2_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_event2_iclr`]
module"] pub type INT_EVENT2_ICLR = crate :: Reg < int_event2_iclr :: INT_EVENT2_ICLR_SPEC > ; # [doc = "Interrupt clear"] pub mod int_event2_iclr ; # [doc = "EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evt_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_mode`]
module"] pub type EVT_MODE = crate :: Reg < evt_mode :: EVT_MODE_SPEC > ; # [doc = "Event Mode"] pub mod evt_mode ; # [doc = "DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`desc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`]
module"] pub type DESC = crate :: Reg < desc :: DESC_SPEC > ; # [doc = "Module Description"] pub mod desc ; # [doc = "CTL0 (rw) register accessor: UART Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl0`]
module"] pub type CTL0 = crate :: Reg < ctl0 :: CTL0_SPEC > ; # [doc = "UART Control Register 0"] pub mod ctl0 ; # [doc = "LCRH (rw) register accessor: UART Line Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcrh`]
module"] pub type LCRH = crate :: Reg < lcrh :: LCRH_SPEC > ; # [doc = "UART Line Control Register"] pub mod lcrh ; # [doc = "STAT (r) register accessor: UART Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"] pub type STAT = crate :: Reg < stat :: STAT_SPEC > ; # [doc = "UART Status Register"] pub mod stat ; # [doc = "IFLS (rw) register accessor: UART Interrupt FIFO Level Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifls::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifls::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifls`]
module"] pub type IFLS = crate :: Reg < ifls :: IFLS_SPEC > ; # [doc = "UART Interrupt FIFO Level Select Register"] pub mod ifls ; # [doc = "IBRD (rw) register accessor: UART Integer Baud-Rate Divisor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibrd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ibrd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibrd`]
module"] pub type IBRD = crate :: Reg < ibrd :: IBRD_SPEC > ; # [doc = "UART Integer Baud-Rate Divisor Register"] pub mod ibrd ; # [doc = "FBRD (rw) register accessor: UART Fractional Baud-Rate Divisor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fbrd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbrd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fbrd`]
module"] pub type FBRD = crate :: Reg < fbrd :: FBRD_SPEC > ; # [doc = "UART Fractional Baud-Rate Divisor Register"] pub mod fbrd ; # [doc = "GFCTL (rw) register accessor: Glitch Filter Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gfctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gfctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gfctl`]
module"] pub type GFCTL = crate :: Reg < gfctl :: GFCTL_SPEC > ; # [doc = "Glitch Filter Control"] pub mod gfctl ; # [doc = "TXDATA (rw) register accessor: UART Transmit Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata`]
module"] pub type TXDATA = crate :: Reg < txdata :: TXDATA_SPEC > ; # [doc = "UART Transmit Data Register"] pub mod txdata ; # [doc = "RXDATA (r) register accessor: UART Receive Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata`]
module"] pub type RXDATA = crate :: Reg < rxdata :: RXDATA_SPEC > ; # [doc = "UART Receive Data Register"] pub mod rxdata ; # [doc = "AMASK (rw) register accessor: Self Address Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`amask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`amask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amask`]
module"] pub type AMASK = crate :: Reg < amask :: AMASK_SPEC > ; # [doc = "Self Address Mask Register"] pub mod amask ; # [doc = "ADDR (rw) register accessor: Self Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"] pub type ADDR = crate :: Reg < addr :: ADDR_SPEC > ; # [doc = "Self Address Register"] pub mod addr ;