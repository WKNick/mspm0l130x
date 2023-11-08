# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { _reserved0 : [u8 ; 0x0400] , # [doc = "0x400 - Subsciber Port 0"] pub fsub_0 : FSUB_0 , # [doc = "0x404 - Subscriber Port 1"] pub fsub_1 : FSUB_1 , _reserved2 : [u8 ; 0x3c] , # [doc = "0x444 - Publisher Port 0"] pub fpub_0 : FPUB_0 , # [doc = "0x448 - Publisher Port 1"] pub fpub_1 : FPUB_1 , _reserved4 : [u8 ; 0x03b4] , # [doc = "0x800 - Power enable"] pub pwren : PWREN , # [doc = "0x804 - Reset Control"] pub rstctl : RSTCTL , _reserved6 : [u8 ; 0x0c] , # [doc = "0x814 - Status Register"] pub stat : STAT , _reserved7 : [u8 ; 0x07e8] , # [doc = "0x1000 - Clock Divider"] pub clkdiv : CLKDIV , _reserved8 : [u8 ; 0x04] , # [doc = "0x1008 - Clock Select for Ultra Low Power peripherals"] pub clksel : CLKSEL , _reserved9 : [u8 ; 0x0c] , # [doc = "0x1018 - Peripheral Debug Control"] pub pdbgctl : PDBGCTL , _reserved10 : [u8 ; 0x04] , # [doc = "0x1020 - Interrupt index"] pub iidx : IIDX , _reserved11 : [u8 ; 0x04] , # [doc = "0x1028 - Interrupt mask"] pub imask : IMASK , _reserved12 : [u8 ; 0x04] , # [doc = "0x1030 - Raw interrupt status"] pub ris : RIS , _reserved13 : [u8 ; 0x04] , # [doc = "0x1038 - Masked interrupt status"] pub mis : MIS , _reserved14 : [u8 ; 0x04] , # [doc = "0x1040 - Interrupt set"] pub iset : ISET , _reserved15 : [u8 ; 0x04] , # [doc = "0x1048 - Interrupt clear"] pub iclr : ICLR , _reserved16 : [u8 ; 0x94] , # [doc = "0x10e0 - Event Mode"] pub evt_mode : EVT_MODE , _reserved17 : [u8 ; 0x18] , # [doc = "0x10fc - Module Description"] pub desc : DESC , # [doc = "0x1100 - CCP Direction"] pub ccpd : CCPD , # [doc = "0x1104 - Output Disable"] pub odis : ODIS , # [doc = "0x1108 - Counter Clock Control Register"] pub cclkctl : CCLKCTL , # [doc = "0x110c - Clock Prescale Register"] pub cps : CPS , # [doc = "0x1110 - Clock prescale count status register"] pub cpsv : CPSV , # [doc = "0x1114 - Timer Cross Trigger Control Register"] pub cttrigctl : CTTRIGCTL , _reserved24 : [u8 ; 0x04] , # [doc = "0x111c - Timer Cross Trigger Register"] pub cttrig : CTTRIG , _reserved25 : [u8 ; 0x06e0] , # [doc = "0x1800 - Counter Register"] pub ctr : CTR , # [doc = "0x1804 - Counter Control Register"] pub ctrctl : CTRCTL , # [doc = "0x1808 - Load Register"] pub load : LOAD , _reserved28 : [u8 ; 0x04] , # [doc = "0x1810..0x1818 - Capture or Compare Register 0 to Capture or Compare Register 1"] pub cc_01 : [CC_01 ; 2] , _reserved29 : [u8 ; 0x18] , # [doc = "0x1830..0x1838 - Capture or Compare Control Registers"] pub ccctl_01 : [CCCTL_01 ; 2] , _reserved30 : [u8 ; 0x18] , # [doc = "0x1850..0x1858 - CCP Output Control Registers"] pub octl_01 : [OCTL_01 ; 2] , _reserved31 : [u8 ; 0x18] , # [doc = "0x1870..0x1878 - Capture or Compare Action Registers"] pub ccact_01 : [CCACT_01 ; 2] , _reserved32 : [u8 ; 0x08] , # [doc = "0x1880..0x1888 - Input Filter Control Register"] pub ifctl_01 : [IFCTL_01 ; 2] , _reserved33 : [u8 ; 0x28] , # [doc = "0x18b0 - Trigger Select"] pub tsel : TSEL , } # [doc = "FSUB_0 (rw) register accessor: Subsciber Port 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsub_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsub_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsub_0`]
module"] pub type FSUB_0 = crate :: Reg < fsub_0 :: FSUB_0_SPEC > ; # [doc = "Subsciber Port 0"] pub mod fsub_0 ; # [doc = "FSUB_1 (rw) register accessor: Subscriber Port 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsub_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsub_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsub_1`]
module"] pub type FSUB_1 = crate :: Reg < fsub_1 :: FSUB_1_SPEC > ; # [doc = "Subscriber Port 1"] pub mod fsub_1 ; # [doc = "FPUB_0 (rw) register accessor: Publisher Port 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpub_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpub_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpub_0`]
module"] pub type FPUB_0 = crate :: Reg < fpub_0 :: FPUB_0_SPEC > ; # [doc = "Publisher Port 0"] pub mod fpub_0 ; # [doc = "FPUB_1 (rw) register accessor: Publisher Port 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpub_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpub_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpub_1`]
module"] pub type FPUB_1 = crate :: Reg < fpub_1 :: FPUB_1_SPEC > ; # [doc = "Publisher Port 1"] pub mod fpub_1 ; # [doc = "PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwren::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwren::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwren`]
module"] pub type PWREN = crate :: Reg < pwren :: PWREN_SPEC > ; # [doc = "Power enable"] pub mod pwren ; # [doc = "RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctl`]
module"] pub type RSTCTL = crate :: Reg < rstctl :: RSTCTL_SPEC > ; # [doc = "Reset Control"] pub mod rstctl ; # [doc = "STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"] pub type STAT = crate :: Reg < stat :: STAT_SPEC > ; # [doc = "Status Register"] pub mod stat ; # [doc = "CLKDIV (rw) register accessor: Clock Divider\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`]
module"] pub type CLKDIV = crate :: Reg < clkdiv :: CLKDIV_SPEC > ; # [doc = "Clock Divider"] pub mod clkdiv ; # [doc = "CLKSEL (rw) register accessor: Clock Select for Ultra Low Power peripherals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel`]
module"] pub type CLKSEL = crate :: Reg < clksel :: CLKSEL_SPEC > ; # [doc = "Clock Select for Ultra Low Power peripherals"] pub mod clksel ; # [doc = "PDBGCTL (rw) register accessor: Peripheral Debug Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdbgctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdbgctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdbgctl`]
module"] pub type PDBGCTL = crate :: Reg < pdbgctl :: PDBGCTL_SPEC > ; # [doc = "Peripheral Debug Control"] pub mod pdbgctl ; # [doc = "IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iidx::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iidx`]
module"] pub type IIDX = crate :: Reg < iidx :: IIDX_SPEC > ; # [doc = "Interrupt index"] pub mod iidx ; # [doc = "IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imask`]
module"] pub type IMASK = crate :: Reg < imask :: IMASK_SPEC > ; # [doc = "Interrupt mask"] pub mod imask ; # [doc = "RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"] pub type RIS = crate :: Reg < ris :: RIS_SPEC > ; # [doc = "Raw interrupt status"] pub mod ris ; # [doc = "MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`]
module"] pub type MIS = crate :: Reg < mis :: MIS_SPEC > ; # [doc = "Masked interrupt status"] pub mod mis ; # [doc = "ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iset`]
module"] pub type ISET = crate :: Reg < iset :: ISET_SPEC > ; # [doc = "Interrupt set"] pub mod iset ; # [doc = "ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iclr`]
module"] pub type ICLR = crate :: Reg < iclr :: ICLR_SPEC > ; # [doc = "Interrupt clear"] pub mod iclr ; # [doc = "EVT_MODE (rw) register accessor: Event Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evt_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_mode`]
module"] pub type EVT_MODE = crate :: Reg < evt_mode :: EVT_MODE_SPEC > ; # [doc = "Event Mode"] pub mod evt_mode ; # [doc = "DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`desc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`]
module"] pub type DESC = crate :: Reg < desc :: DESC_SPEC > ; # [doc = "Module Description"] pub mod desc ; # [doc = "CCPD (rw) register accessor: CCP Direction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccpd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccpd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccpd`]
module"] pub type CCPD = crate :: Reg < ccpd :: CCPD_SPEC > ; # [doc = "CCP Direction"] pub mod ccpd ; # [doc = "ODIS (rw) register accessor: Output Disable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odis`]
module"] pub type ODIS = crate :: Reg < odis :: ODIS_SPEC > ; # [doc = "Output Disable"] pub mod odis ; # [doc = "CCLKCTL (rw) register accessor: Counter Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cclkctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cclkctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cclkctl`]
module"] pub type CCLKCTL = crate :: Reg < cclkctl :: CCLKCTL_SPEC > ; # [doc = "Counter Clock Control Register"] pub mod cclkctl ; # [doc = "CPS (rw) register accessor: Clock Prescale Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cps::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cps::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cps`]
module"] pub type CPS = crate :: Reg < cps :: CPS_SPEC > ; # [doc = "Clock Prescale Register"] pub mod cps ; # [doc = "CPSV (r) register accessor: Clock prescale count status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsv::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsv`]
module"] pub type CPSV = crate :: Reg < cpsv :: CPSV_SPEC > ; # [doc = "Clock prescale count status register"] pub mod cpsv ; # [doc = "CTTRIGCTL (rw) register accessor: Timer Cross Trigger Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cttrigctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cttrigctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cttrigctl`]
module"] pub type CTTRIGCTL = crate :: Reg < cttrigctl :: CTTRIGCTL_SPEC > ; # [doc = "Timer Cross Trigger Control Register"] pub mod cttrigctl ; # [doc = "CTTRIG (w) register accessor: Timer Cross Trigger Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cttrig::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cttrig`]
module"] pub type CTTRIG = crate :: Reg < cttrig :: CTTRIG_SPEC > ; # [doc = "Timer Cross Trigger Register"] pub mod cttrig ; # [doc = "CTR (rw) register accessor: Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctr`]
module"] pub type CTR = crate :: Reg < ctr :: CTR_SPEC > ; # [doc = "Counter Register"] pub mod ctr ; # [doc = "CTRCTL (rw) register accessor: Counter Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrctl`]
module"] pub type CTRCTL = crate :: Reg < ctrctl :: CTRCTL_SPEC > ; # [doc = "Counter Control Register"] pub mod ctrctl ; # [doc = "LOAD (rw) register accessor: Load Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`load::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`load::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@load`]
module"] pub type LOAD = crate :: Reg < load :: LOAD_SPEC > ; # [doc = "Load Register"] pub mod load ; # [doc = "CC_01 (rw) register accessor: Capture or Compare Register 0 to Capture or Compare Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc_01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc_01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc_01`]
module"] pub type CC_01 = crate :: Reg < cc_01 :: CC_01_SPEC > ; # [doc = "Capture or Compare Register 0 to Capture or Compare Register 1"] pub mod cc_01 ; # [doc = "CCCTL_01 (rw) register accessor: Capture or Compare Control Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccctl_01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccctl_01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccctl_01`]
module"] pub type CCCTL_01 = crate :: Reg < ccctl_01 :: CCCTL_01_SPEC > ; # [doc = "Capture or Compare Control Registers"] pub mod ccctl_01 ; # [doc = "OCTL_01 (rw) register accessor: CCP Output Control Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`octl_01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`octl_01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@octl_01`]
module"] pub type OCTL_01 = crate :: Reg < octl_01 :: OCTL_01_SPEC > ; # [doc = "CCP Output Control Registers"] pub mod octl_01 ; # [doc = "CCACT_01 (rw) register accessor: Capture or Compare Action Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccact_01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccact_01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccact_01`]
module"] pub type CCACT_01 = crate :: Reg < ccact_01 :: CCACT_01_SPEC > ; # [doc = "Capture or Compare Action Registers"] pub mod ccact_01 ; # [doc = "IFCTL_01 (rw) register accessor: Input Filter Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifctl_01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifctl_01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifctl_01`]
module"] pub type IFCTL_01 = crate :: Reg < ifctl_01 :: IFCTL_01_SPEC > ; # [doc = "Input Filter Control Register"] pub mod ifctl_01 ; # [doc = "TSEL (rw) register accessor: Trigger Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsel`]
module"] pub type TSEL = crate :: Reg < tsel :: TSEL_SPEC > ; # [doc = "Trigger Select"] pub mod tsel ;