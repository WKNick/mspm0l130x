# [doc = "Register `TXD` reader"] pub type R = crate :: R < TXD_SPEC > ; # [doc = "Field `TXD_TX_DATA` reader - Contains data written by an external debug tool to the SEC-AP TXDATA register"] pub type TXD_TX_DATA_R = crate :: FieldReader < u32 > ; impl R { # [doc = "Bits 0:31 - Contains data written by an external debug tool to the SEC-AP TXDATA register"] # [inline (always)] pub fn txd_tx_data (& self) -> TXD_TX_DATA_R { TXD_TX_DATA_R :: new (self . bits) } } # [doc = "Transmit data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct TXD_SPEC ; impl crate :: RegisterSpec for TXD_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`txd::R`](R) reader structure"] impl crate :: Readable for TXD_SPEC { } # [doc = "`reset()` method sets TXD to value 0"] impl crate :: Resettable for TXD_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }