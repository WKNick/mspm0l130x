# [doc = "Register `INT_EVENT0_ICLR` writer"] pub type W = crate :: W < INT_EVENT0_ICLR_SPEC > ; # [doc = "Clear UARTOUT Receive Time-Out Interrupt.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_ICLR_RTOUT_AW { # [doc = "0: NO_EFFECT"] INT_EVENT0_ICLR_RTOUT_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT0_ICLR_RTOUT_CLR = 1 , } impl From < INT_EVENT0_ICLR_RTOUT_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT0_ICLR_RTOUT_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT0_ICLR_RTOUT` writer - Clear UARTOUT Receive Time-Out Interrupt."] pub type INT_EVENT0_ICLR_RTOUT_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_ICLR_RTOUT_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT0_ICLR_RTOUT_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event0_iclr_rtout_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_RTOUT_AW :: INT_EVENT0_ICLR_RTOUT_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event0_iclr_rtout_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_RTOUT_AW :: INT_EVENT0_ICLR_RTOUT_CLR) } } # [doc = "Clear UART Framing Error Interrupt.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_ICLR_FRMERR_AW { # [doc = "0: NO_EFFECT"] INT_EVENT0_ICLR_FRMERR_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT0_ICLR_FRMERR_CLR = 1 , } impl From < INT_EVENT0_ICLR_FRMERR_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT0_ICLR_FRMERR_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT0_ICLR_FRMERR` writer - Clear UART Framing Error Interrupt."] pub type INT_EVENT0_ICLR_FRMERR_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_ICLR_FRMERR_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT0_ICLR_FRMERR_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event0_iclr_frmerr_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_FRMERR_AW :: INT_EVENT0_ICLR_FRMERR_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event0_iclr_frmerr_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_FRMERR_AW :: INT_EVENT0_ICLR_FRMERR_CLR) } } # [doc = "Clear UART Parity Error Interrupt.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_ICLR_PARERR_AW { # [doc = "0: NO_EFFECT"] INT_EVENT0_ICLR_PARERR_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT0_ICLR_PARERR_CLR = 1 , } impl From < INT_EVENT0_ICLR_PARERR_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT0_ICLR_PARERR_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT0_ICLR_PARERR` writer - Clear UART Parity Error Interrupt."] pub type INT_EVENT0_ICLR_PARERR_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_ICLR_PARERR_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT0_ICLR_PARERR_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event0_iclr_parerr_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_PARERR_AW :: INT_EVENT0_ICLR_PARERR_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event0_iclr_parerr_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_PARERR_AW :: INT_EVENT0_ICLR_PARERR_CLR) } } # [doc = "Clear UART Break Error Interrupt.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_ICLR_BRKERR_AW { # [doc = "0: NO_EFFECT"] INT_EVENT0_ICLR_BRKERR_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT0_ICLR_BRKERR_CLR = 1 , } impl From < INT_EVENT0_ICLR_BRKERR_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT0_ICLR_BRKERR_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT0_ICLR_BRKERR` writer - Clear UART Break Error Interrupt."] pub type INT_EVENT0_ICLR_BRKERR_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_ICLR_BRKERR_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT0_ICLR_BRKERR_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event0_iclr_brkerr_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_BRKERR_AW :: INT_EVENT0_ICLR_BRKERR_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event0_iclr_brkerr_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_BRKERR_AW :: INT_EVENT0_ICLR_BRKERR_CLR) } } # [doc = "Clear UART Receive Overrun Error Interrupt.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_ICLR_OVRERR_AW { # [doc = "0: NO_EFFECT"] INT_EVENT0_ICLR_OVRERR_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT0_ICLR_OVRERR_CLR = 1 , } impl From < INT_EVENT0_ICLR_OVRERR_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT0_ICLR_OVRERR_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT0_ICLR_OVRERR` writer - Clear UART Receive Overrun Error Interrupt."] pub type INT_EVENT0_ICLR_OVRERR_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_ICLR_OVRERR_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT0_ICLR_OVRERR_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event0_iclr_ovrerr_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_OVRERR_AW :: INT_EVENT0_ICLR_OVRERR_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event0_iclr_ovrerr_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_OVRERR_AW :: INT_EVENT0_ICLR_OVRERR_CLR) } } # [doc = "Clear Negative Edge on UARTxRXD Interrupt.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_ICLR_RXNE_AW { # [doc = "0: NO_EFFECT"] INT_EVENT0_ICLR_RXNE_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT0_ICLR_RXNE_CLR = 1 , } impl From < INT_EVENT0_ICLR_RXNE_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT0_ICLR_RXNE_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT0_ICLR_RXNE` writer - Clear Negative Edge on UARTxRXD Interrupt."] pub type INT_EVENT0_ICLR_RXNE_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_ICLR_RXNE_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT0_ICLR_RXNE_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event0_iclr_rxne_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_RXNE_AW :: INT_EVENT0_ICLR_RXNE_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event0_iclr_rxne_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_RXNE_AW :: INT_EVENT0_ICLR_RXNE_CLR) } } # [doc = "Clear Positive Edge on UARTxRXD Interrupt.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_ICLR_RXPE_AW { # [doc = "0: NO_EFFECT"] INT_EVENT0_ICLR_RXPE_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT0_ICLR_RXPE_CLR = 1 , } impl From < INT_EVENT0_ICLR_RXPE_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT0_ICLR_RXPE_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT0_ICLR_RXPE` writer - Clear Positive Edge on UARTxRXD Interrupt."] pub type INT_EVENT0_ICLR_RXPE_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_ICLR_RXPE_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT0_ICLR_RXPE_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event0_iclr_rxpe_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_RXPE_AW :: INT_EVENT0_ICLR_RXPE_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event0_iclr_rxpe_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_RXPE_AW :: INT_EVENT0_ICLR_RXPE_CLR) } } # [doc = "Clear LIN Capture 0 / Match Interrupt .\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_ICLR_LINC0_AW { # [doc = "0: NO_EFFECT"] INT_EVENT0_ICLR_LINC0_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT0_ICLR_LINC0_CLR = 1 , } impl From < INT_EVENT0_ICLR_LINC0_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT0_ICLR_LINC0_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT0_ICLR_LINC0` writer - Clear LIN Capture 0 / Match Interrupt ."] pub type INT_EVENT0_ICLR_LINC0_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_ICLR_LINC0_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT0_ICLR_LINC0_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event0_iclr_linc0_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_LINC0_AW :: INT_EVENT0_ICLR_LINC0_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event0_iclr_linc0_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_LINC0_AW :: INT_EVENT0_ICLR_LINC0_CLR) } } # [doc = "Clear LIN Capture 1 Interrupt.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_ICLR_LINC1_AW { # [doc = "0: NO_EFFECT"] INT_EVENT0_ICLR_LINC1_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT0_ICLR_LINC1_CLR = 1 , } impl From < INT_EVENT0_ICLR_LINC1_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT0_ICLR_LINC1_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT0_ICLR_LINC1` writer - Clear LIN Capture 1 Interrupt."] pub type INT_EVENT0_ICLR_LINC1_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_ICLR_LINC1_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT0_ICLR_LINC1_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event0_iclr_linc1_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_LINC1_AW :: INT_EVENT0_ICLR_LINC1_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event0_iclr_linc1_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_LINC1_AW :: INT_EVENT0_ICLR_LINC1_CLR) } } # [doc = "Clear LIN Hardware Counter Overflow Interrupt.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_ICLR_LINOVF_AW { # [doc = "0: NO_EFFECT"] INT_EVENT0_ICLR_LINOVF_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT0_ICLR_LINOVF_CLR = 1 , } impl From < INT_EVENT0_ICLR_LINOVF_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT0_ICLR_LINOVF_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT0_ICLR_LINOVF` writer - Clear LIN Hardware Counter Overflow Interrupt."] pub type INT_EVENT0_ICLR_LINOVF_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_ICLR_LINOVF_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT0_ICLR_LINOVF_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event0_iclr_linovf_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_LINOVF_AW :: INT_EVENT0_ICLR_LINOVF_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event0_iclr_linovf_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_LINOVF_AW :: INT_EVENT0_ICLR_LINOVF_CLR) } } # [doc = "Clear UART Receive Interrupt.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_ICLR_RXINT_AW { # [doc = "0: NO_EFFECT"] INT_EVENT0_ICLR_RXINT_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT0_ICLR_RXINT_CLR = 1 , } impl From < INT_EVENT0_ICLR_RXINT_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT0_ICLR_RXINT_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT0_ICLR_RXINT` writer - Clear UART Receive Interrupt."] pub type INT_EVENT0_ICLR_RXINT_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_ICLR_RXINT_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT0_ICLR_RXINT_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event0_iclr_rxint_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_RXINT_AW :: INT_EVENT0_ICLR_RXINT_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event0_iclr_rxint_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_RXINT_AW :: INT_EVENT0_ICLR_RXINT_CLR) } } # [doc = "Clear UART Transmit Interrupt.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_ICLR_TXINT_AW { # [doc = "0: NO_EFFECT"] INT_EVENT0_ICLR_TXINT_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT0_ICLR_TXINT_CLR = 1 , } impl From < INT_EVENT0_ICLR_TXINT_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT0_ICLR_TXINT_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT0_ICLR_TXINT` writer - Clear UART Transmit Interrupt."] pub type INT_EVENT0_ICLR_TXINT_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_ICLR_TXINT_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT0_ICLR_TXINT_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event0_iclr_txint_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_TXINT_AW :: INT_EVENT0_ICLR_TXINT_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event0_iclr_txint_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_TXINT_AW :: INT_EVENT0_ICLR_TXINT_CLR) } } # [doc = "Clear UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_ICLR_EOT_AW { # [doc = "0: NO_EFFECT"] INT_EVENT0_ICLR_EOT_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT0_ICLR_EOT_CLR = 1 , } impl From < INT_EVENT0_ICLR_EOT_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT0_ICLR_EOT_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT0_ICLR_EOT` writer - Clear UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."] pub type INT_EVENT0_ICLR_EOT_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_ICLR_EOT_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT0_ICLR_EOT_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event0_iclr_eot_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_EOT_AW :: INT_EVENT0_ICLR_EOT_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event0_iclr_eot_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_EOT_AW :: INT_EVENT0_ICLR_EOT_CLR) } } # [doc = "Clear Address Match Interrupt.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_ICLR_ADDR_MATCH_AW { # [doc = "0: NO_EFFECT"] INT_EVENT0_ICLR_ADDR_MATCH_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT0_ICLR_ADDR_MATCH_CLR = 1 , } impl From < INT_EVENT0_ICLR_ADDR_MATCH_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT0_ICLR_ADDR_MATCH_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT0_ICLR_ADDR_MATCH` writer - Clear Address Match Interrupt."] pub type INT_EVENT0_ICLR_ADDR_MATCH_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_ICLR_ADDR_MATCH_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT0_ICLR_ADDR_MATCH_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event0_iclr_addr_match_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_ADDR_MATCH_AW :: INT_EVENT0_ICLR_ADDR_MATCH_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event0_iclr_addr_match_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_ADDR_MATCH_AW :: INT_EVENT0_ICLR_ADDR_MATCH_CLR) } } # [doc = "Clear UART Clear to Send Modem Interrupt. 0 = Interrupt disabled\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_ICLR_CTS_AW { # [doc = "0: NO_EFFECT"] INT_EVENT0_ICLR_CTS_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT0_ICLR_CTS_CLR = 1 , } impl From < INT_EVENT0_ICLR_CTS_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT0_ICLR_CTS_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT0_ICLR_CTS` writer - Clear UART Clear to Send Modem Interrupt. 0 = Interrupt disabled"] pub type INT_EVENT0_ICLR_CTS_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_ICLR_CTS_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT0_ICLR_CTS_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event0_iclr_cts_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_CTS_AW :: INT_EVENT0_ICLR_CTS_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event0_iclr_cts_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_CTS_AW :: INT_EVENT0_ICLR_CTS_CLR) } } # [doc = "Clear DMA Done on RX Event Channel\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_ICLR_DMA_DONE_RX_AW { # [doc = "0: NO_EFFECT"] INT_EVENT0_ICLR_DMA_DONE_RX_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT0_ICLR_DMA_DONE_RX_CLR = 1 , } impl From < INT_EVENT0_ICLR_DMA_DONE_RX_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT0_ICLR_DMA_DONE_RX_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT0_ICLR_DMA_DONE_RX` writer - Clear DMA Done on RX Event Channel"] pub type INT_EVENT0_ICLR_DMA_DONE_RX_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_ICLR_DMA_DONE_RX_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT0_ICLR_DMA_DONE_RX_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event0_iclr_dma_done_rx_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_DMA_DONE_RX_AW :: INT_EVENT0_ICLR_DMA_DONE_RX_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event0_iclr_dma_done_rx_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_DMA_DONE_RX_AW :: INT_EVENT0_ICLR_DMA_DONE_RX_CLR) } } # [doc = "Clear DMA Done on TX Event Channel\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_ICLR_DMA_DONE_TX_AW { # [doc = "0: NO_EFFECT"] INT_EVENT0_ICLR_DMA_DONE_TX_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT0_ICLR_DMA_DONE_TX_CLR = 1 , } impl From < INT_EVENT0_ICLR_DMA_DONE_TX_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT0_ICLR_DMA_DONE_TX_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT0_ICLR_DMA_DONE_TX` writer - Clear DMA Done on TX Event Channel"] pub type INT_EVENT0_ICLR_DMA_DONE_TX_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_ICLR_DMA_DONE_TX_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT0_ICLR_DMA_DONE_TX_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event0_iclr_dma_done_tx_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_DMA_DONE_TX_AW :: INT_EVENT0_ICLR_DMA_DONE_TX_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event0_iclr_dma_done_tx_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_DMA_DONE_TX_AW :: INT_EVENT0_ICLR_DMA_DONE_TX_CLR) } } # [doc = "Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT0_ICLR_NERR_AW { # [doc = "0: NO_EFFECT"] INT_EVENT0_ICLR_NERR_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT0_ICLR_NERR_CLR = 1 , } impl From < INT_EVENT0_ICLR_NERR_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT0_ICLR_NERR_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT0_ICLR_NERR` writer - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"] pub type INT_EVENT0_ICLR_NERR_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT0_ICLR_NERR_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT0_ICLR_NERR_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event0_iclr_nerr_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_NERR_AW :: INT_EVENT0_ICLR_NERR_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event0_iclr_nerr_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT0_ICLR_NERR_AW :: INT_EVENT0_ICLR_NERR_CLR) } } impl W { # [doc = "Bit 0 - Clear UARTOUT Receive Time-Out Interrupt."] # [inline (always)] # [must_use] pub fn int_event0_iclr_rtout (& mut self) -> INT_EVENT0_ICLR_RTOUT_W < INT_EVENT0_ICLR_SPEC , 0 > { INT_EVENT0_ICLR_RTOUT_W :: new (self) } # [doc = "Bit 1 - Clear UART Framing Error Interrupt."] # [inline (always)] # [must_use] pub fn int_event0_iclr_frmerr (& mut self) -> INT_EVENT0_ICLR_FRMERR_W < INT_EVENT0_ICLR_SPEC , 1 > { INT_EVENT0_ICLR_FRMERR_W :: new (self) } # [doc = "Bit 2 - Clear UART Parity Error Interrupt."] # [inline (always)] # [must_use] pub fn int_event0_iclr_parerr (& mut self) -> INT_EVENT0_ICLR_PARERR_W < INT_EVENT0_ICLR_SPEC , 2 > { INT_EVENT0_ICLR_PARERR_W :: new (self) } # [doc = "Bit 3 - Clear UART Break Error Interrupt."] # [inline (always)] # [must_use] pub fn int_event0_iclr_brkerr (& mut self) -> INT_EVENT0_ICLR_BRKERR_W < INT_EVENT0_ICLR_SPEC , 3 > { INT_EVENT0_ICLR_BRKERR_W :: new (self) } # [doc = "Bit 4 - Clear UART Receive Overrun Error Interrupt."] # [inline (always)] # [must_use] pub fn int_event0_iclr_ovrerr (& mut self) -> INT_EVENT0_ICLR_OVRERR_W < INT_EVENT0_ICLR_SPEC , 4 > { INT_EVENT0_ICLR_OVRERR_W :: new (self) } # [doc = "Bit 5 - Clear Negative Edge on UARTxRXD Interrupt."] # [inline (always)] # [must_use] pub fn int_event0_iclr_rxne (& mut self) -> INT_EVENT0_ICLR_RXNE_W < INT_EVENT0_ICLR_SPEC , 5 > { INT_EVENT0_ICLR_RXNE_W :: new (self) } # [doc = "Bit 6 - Clear Positive Edge on UARTxRXD Interrupt."] # [inline (always)] # [must_use] pub fn int_event0_iclr_rxpe (& mut self) -> INT_EVENT0_ICLR_RXPE_W < INT_EVENT0_ICLR_SPEC , 6 > { INT_EVENT0_ICLR_RXPE_W :: new (self) } # [doc = "Bit 7 - Clear LIN Capture 0 / Match Interrupt ."] # [inline (always)] # [must_use] pub fn int_event0_iclr_linc0 (& mut self) -> INT_EVENT0_ICLR_LINC0_W < INT_EVENT0_ICLR_SPEC , 7 > { INT_EVENT0_ICLR_LINC0_W :: new (self) } # [doc = "Bit 8 - Clear LIN Capture 1 Interrupt."] # [inline (always)] # [must_use] pub fn int_event0_iclr_linc1 (& mut self) -> INT_EVENT0_ICLR_LINC1_W < INT_EVENT0_ICLR_SPEC , 8 > { INT_EVENT0_ICLR_LINC1_W :: new (self) } # [doc = "Bit 9 - Clear LIN Hardware Counter Overflow Interrupt."] # [inline (always)] # [must_use] pub fn int_event0_iclr_linovf (& mut self) -> INT_EVENT0_ICLR_LINOVF_W < INT_EVENT0_ICLR_SPEC , 9 > { INT_EVENT0_ICLR_LINOVF_W :: new (self) } # [doc = "Bit 10 - Clear UART Receive Interrupt."] # [inline (always)] # [must_use] pub fn int_event0_iclr_rxint (& mut self) -> INT_EVENT0_ICLR_RXINT_W < INT_EVENT0_ICLR_SPEC , 10 > { INT_EVENT0_ICLR_RXINT_W :: new (self) } # [doc = "Bit 11 - Clear UART Transmit Interrupt."] # [inline (always)] # [must_use] pub fn int_event0_iclr_txint (& mut self) -> INT_EVENT0_ICLR_TXINT_W < INT_EVENT0_ICLR_SPEC , 11 > { INT_EVENT0_ICLR_TXINT_W :: new (self) } # [doc = "Bit 12 - Clear UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."] # [inline (always)] # [must_use] pub fn int_event0_iclr_eot (& mut self) -> INT_EVENT0_ICLR_EOT_W < INT_EVENT0_ICLR_SPEC , 12 > { INT_EVENT0_ICLR_EOT_W :: new (self) } # [doc = "Bit 13 - Clear Address Match Interrupt."] # [inline (always)] # [must_use] pub fn int_event0_iclr_addr_match (& mut self) -> INT_EVENT0_ICLR_ADDR_MATCH_W < INT_EVENT0_ICLR_SPEC , 13 > { INT_EVENT0_ICLR_ADDR_MATCH_W :: new (self) } # [doc = "Bit 14 - Clear UART Clear to Send Modem Interrupt. 0 = Interrupt disabled"] # [inline (always)] # [must_use] pub fn int_event0_iclr_cts (& mut self) -> INT_EVENT0_ICLR_CTS_W < INT_EVENT0_ICLR_SPEC , 14 > { INT_EVENT0_ICLR_CTS_W :: new (self) } # [doc = "Bit 15 - Clear DMA Done on RX Event Channel"] # [inline (always)] # [must_use] pub fn int_event0_iclr_dma_done_rx (& mut self) -> INT_EVENT0_ICLR_DMA_DONE_RX_W < INT_EVENT0_ICLR_SPEC , 15 > { INT_EVENT0_ICLR_DMA_DONE_RX_W :: new (self) } # [doc = "Bit 16 - Clear DMA Done on TX Event Channel"] # [inline (always)] # [must_use] pub fn int_event0_iclr_dma_done_tx (& mut self) -> INT_EVENT0_ICLR_DMA_DONE_TX_W < INT_EVENT0_ICLR_SPEC , 16 > { INT_EVENT0_ICLR_DMA_DONE_TX_W :: new (self) } # [doc = "Bit 17 - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"] # [inline (always)] # [must_use] pub fn int_event0_iclr_nerr (& mut self) -> INT_EVENT0_ICLR_NERR_W < INT_EVENT0_ICLR_SPEC , 17 > { INT_EVENT0_ICLR_NERR_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Interrupt clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_event0_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct INT_EVENT0_ICLR_SPEC ; impl crate :: RegisterSpec for INT_EVENT0_ICLR_SPEC { type Ux = u32 ; } # [doc = "`write(|w| ..)` method takes [`int_event0_iclr::W`](W) writer structure"] impl crate :: Writable for INT_EVENT0_ICLR_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets INT_EVENT0_ICLR to value 0"] impl crate :: Resettable for INT_EVENT0_ICLR_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }