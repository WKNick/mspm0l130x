# [doc = "Register `INT_EVENT1_IMASK` reader"] pub type R = crate :: R < INT_EVENT1_IMASK_SPEC > ; # [doc = "Register `INT_EVENT1_IMASK` writer"] pub type W = crate :: W < INT_EVENT1_IMASK_SPEC > ; # [doc = "Field `INT_EVENT1_IMASK_RTOUT` reader - SPI Receive Time-Out event mask."] pub type INT_EVENT1_IMASK_RTOUT_R = crate :: BitReader < INT_EVENT1_IMASK_RTOUT_A > ; # [doc = "SPI Receive Time-Out event mask.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_IMASK_RTOUT_A { # [doc = "0: CLR"] INT_EVENT1_IMASK_RTOUT_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_IMASK_RTOUT_SET = 1 , } impl From < INT_EVENT1_IMASK_RTOUT_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_IMASK_RTOUT_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_IMASK_RTOUT_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_IMASK_RTOUT_A { match self . bits { false => INT_EVENT1_IMASK_RTOUT_A :: INT_EVENT1_IMASK_RTOUT_CLR , true => INT_EVENT1_IMASK_RTOUT_A :: INT_EVENT1_IMASK_RTOUT_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_imask_rtout_clr (& self) -> bool { * self == INT_EVENT1_IMASK_RTOUT_A :: INT_EVENT1_IMASK_RTOUT_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_imask_rtout_set (& self) -> bool { * self == INT_EVENT1_IMASK_RTOUT_A :: INT_EVENT1_IMASK_RTOUT_SET } } # [doc = "Field `INT_EVENT1_IMASK_RTOUT` writer - SPI Receive Time-Out event mask."] pub type INT_EVENT1_IMASK_RTOUT_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT1_IMASK_RTOUT_A > ; impl < 'a , REG , const O : u8 > INT_EVENT1_IMASK_RTOUT_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event1_imask_rtout_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_RTOUT_A :: INT_EVENT1_IMASK_RTOUT_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event1_imask_rtout_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_RTOUT_A :: INT_EVENT1_IMASK_RTOUT_SET) } } # [doc = "Field `INT_EVENT1_IMASK_RX` reader - Receive FIFO event mask."] pub type INT_EVENT1_IMASK_RX_R = crate :: BitReader < INT_EVENT1_IMASK_RX_A > ; # [doc = "Receive FIFO event mask.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_IMASK_RX_A { # [doc = "0: CLR"] INT_EVENT1_IMASK_RX_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_IMASK_RX_SET = 1 , } impl From < INT_EVENT1_IMASK_RX_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_IMASK_RX_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_IMASK_RX_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_IMASK_RX_A { match self . bits { false => INT_EVENT1_IMASK_RX_A :: INT_EVENT1_IMASK_RX_CLR , true => INT_EVENT1_IMASK_RX_A :: INT_EVENT1_IMASK_RX_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_imask_rx_clr (& self) -> bool { * self == INT_EVENT1_IMASK_RX_A :: INT_EVENT1_IMASK_RX_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_imask_rx_set (& self) -> bool { * self == INT_EVENT1_IMASK_RX_A :: INT_EVENT1_IMASK_RX_SET } } # [doc = "Field `INT_EVENT1_IMASK_RX` writer - Receive FIFO event mask."] pub type INT_EVENT1_IMASK_RX_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT1_IMASK_RX_A > ; impl < 'a , REG , const O : u8 > INT_EVENT1_IMASK_RX_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event1_imask_rx_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_RX_A :: INT_EVENT1_IMASK_RX_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event1_imask_rx_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_RX_A :: INT_EVENT1_IMASK_RX_SET) } } impl R { # [doc = "Bit 2 - SPI Receive Time-Out event mask."] # [inline (always)] pub fn int_event1_imask_rtout (& self) -> INT_EVENT1_IMASK_RTOUT_R { INT_EVENT1_IMASK_RTOUT_R :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 3 - Receive FIFO event mask."] # [inline (always)] pub fn int_event1_imask_rx (& self) -> INT_EVENT1_IMASK_RX_R { INT_EVENT1_IMASK_RX_R :: new (((self . bits >> 3) & 1) != 0) } } impl W { # [doc = "Bit 2 - SPI Receive Time-Out event mask."] # [inline (always)] # [must_use] pub fn int_event1_imask_rtout (& mut self) -> INT_EVENT1_IMASK_RTOUT_W < INT_EVENT1_IMASK_SPEC , 2 > { INT_EVENT1_IMASK_RTOUT_W :: new (self) } # [doc = "Bit 3 - Receive FIFO event mask."] # [inline (always)] # [must_use] pub fn int_event1_imask_rx (& mut self) -> INT_EVENT1_IMASK_RX_W < INT_EVENT1_IMASK_SPEC , 3 > { INT_EVENT1_IMASK_RX_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_event1_imask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_event1_imask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct INT_EVENT1_IMASK_SPEC ; impl crate :: RegisterSpec for INT_EVENT1_IMASK_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`int_event1_imask::R`](R) reader structure"] impl crate :: Readable for INT_EVENT1_IMASK_SPEC { } # [doc = "`write(|w| ..)` method takes [`int_event1_imask::W`](W) writer structure"] impl crate :: Writable for INT_EVENT1_IMASK_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets INT_EVENT1_IMASK to value 0"] impl crate :: Resettable for INT_EVENT1_IMASK_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }