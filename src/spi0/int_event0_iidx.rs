# [doc = "Register `INT_EVENT0_IIDX` reader"] pub type R = crate :: R < INT_EVENT0_IIDX_SPEC > ; # [doc = "Field `INT_EVENT0_IIDX_STAT` reader - Interrupt index status"] pub type INT_EVENT0_IIDX_STAT_R = crate :: FieldReader < INT_EVENT0_IIDX_STAT_A > ; # [doc = "Interrupt index status\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum INT_EVENT0_IIDX_STAT_A { # [doc = "0: NO_INTR"] INT_EVENT0_IIDX_STAT_NO_INTR = 0 , # [doc = "1: RXFIFO_OFV_EVT"] INT_EVENT0_IIDX_STAT_RXFIFO_OFV_EVT = 1 , # [doc = "2: PER_EVT"] INT_EVENT0_IIDX_STAT_PER_EVT = 2 , # [doc = "3: RTOUT_EVT"] INT_EVENT0_IIDX_STAT_RTOUT_EVT = 3 , # [doc = "4: RX_EVT"] INT_EVENT0_IIDX_STAT_RX_EVT = 4 , # [doc = "5: TX_EVT"] INT_EVENT0_IIDX_STAT_TX_EVT = 5 , # [doc = "6: TX_EMPTY"] INT_EVENT0_IIDX_STAT_TX_EMPTY = 6 , # [doc = "7: IDLE_EVT"] INT_EVENT0_IIDX_STAT_IDLE_EVT = 7 , # [doc = "8: DMA_DONE_RX_EVT"] INT_EVENT0_IIDX_STAT_DMA_DONE_RX_EVT = 8 , # [doc = "9: DMA_DONE_TX_EVT"] INT_EVENT0_IIDX_STAT_DMA_DONE_TX_EVT = 9 , # [doc = "10: TXFIFO_UNF_EVT"] INT_EVENT0_IIDX_STAT_TXFIFO_UNF_EVT = 10 , # [doc = "11: RXFULL_EVT"] INT_EVENT0_IIDX_STAT_RXFULL_EVT = 11 , } impl From < INT_EVENT0_IIDX_STAT_A > for u8 { # [inline (always)] fn from (variant : INT_EVENT0_IIDX_STAT_A) -> Self { variant as _ } } impl crate :: FieldSpec for INT_EVENT0_IIDX_STAT_A { type Ux = u8 ; } impl INT_EVENT0_IIDX_STAT_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < INT_EVENT0_IIDX_STAT_A > { match self . bits { 0 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_NO_INTR) , 1 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_RXFIFO_OFV_EVT) , 2 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_PER_EVT) , 3 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_RTOUT_EVT) , 4 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_RX_EVT) , 5 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_TX_EVT) , 6 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_TX_EMPTY) , 7 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_IDLE_EVT) , 8 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_DMA_DONE_RX_EVT) , 9 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_DMA_DONE_TX_EVT) , 10 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_TXFIFO_UNF_EVT) , 11 => Some (INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_RXFULL_EVT) , _ => None , } } # [doc = "NO_INTR"] # [inline (always)] pub fn is_int_event0_iidx_stat_no_intr (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_NO_INTR } # [doc = "RXFIFO_OFV_EVT"] # [inline (always)] pub fn is_int_event0_iidx_stat_rxfifo_ofv_evt (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_RXFIFO_OFV_EVT } # [doc = "PER_EVT"] # [inline (always)] pub fn is_int_event0_iidx_stat_per_evt (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_PER_EVT } # [doc = "RTOUT_EVT"] # [inline (always)] pub fn is_int_event0_iidx_stat_rtout_evt (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_RTOUT_EVT } # [doc = "RX_EVT"] # [inline (always)] pub fn is_int_event0_iidx_stat_rx_evt (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_RX_EVT } # [doc = "TX_EVT"] # [inline (always)] pub fn is_int_event0_iidx_stat_tx_evt (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_TX_EVT } # [doc = "TX_EMPTY"] # [inline (always)] pub fn is_int_event0_iidx_stat_tx_empty (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_TX_EMPTY } # [doc = "IDLE_EVT"] # [inline (always)] pub fn is_int_event0_iidx_stat_idle_evt (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_IDLE_EVT } # [doc = "DMA_DONE_RX_EVT"] # [inline (always)] pub fn is_int_event0_iidx_stat_dma_done_rx_evt (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_DMA_DONE_RX_EVT } # [doc = "DMA_DONE_TX_EVT"] # [inline (always)] pub fn is_int_event0_iidx_stat_dma_done_tx_evt (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_DMA_DONE_TX_EVT } # [doc = "TXFIFO_UNF_EVT"] # [inline (always)] pub fn is_int_event0_iidx_stat_txfifo_unf_evt (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_TXFIFO_UNF_EVT } # [doc = "RXFULL_EVT"] # [inline (always)] pub fn is_int_event0_iidx_stat_rxfull_evt (& self) -> bool { * self == INT_EVENT0_IIDX_STAT_A :: INT_EVENT0_IIDX_STAT_RXFULL_EVT } } impl R { # [doc = "Bits 0:7 - Interrupt index status"] # [inline (always)] pub fn int_event0_iidx_stat (& self) -> INT_EVENT0_IIDX_STAT_R { INT_EVENT0_IIDX_STAT_R :: new ((self . bits & 0xff) as u8) } } # [doc = "Interrupt Index Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_event0_iidx::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct INT_EVENT0_IIDX_SPEC ; impl crate :: RegisterSpec for INT_EVENT0_IIDX_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`int_event0_iidx::R`](R) reader structure"] impl crate :: Readable for INT_EVENT0_IIDX_SPEC { } # [doc = "`reset()` method sets INT_EVENT0_IIDX to value 0"] impl crate :: Resettable for INT_EVENT0_IIDX_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }