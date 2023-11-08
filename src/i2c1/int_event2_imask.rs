# [doc = "Register `INT_EVENT2_IMASK` reader"] pub type R = crate :: R < INT_EVENT2_IMASK_SPEC > ; # [doc = "Register `INT_EVENT2_IMASK` writer"] pub type W = crate :: W < INT_EVENT2_IMASK_SPEC > ; # [doc = "Field `INT_EVENT2_IMASK_MRXFIFOTRG` reader - Master Receive FIFO Trigger Trigger when RX FIFO contains &amp;gt;= defined bytes"] pub type INT_EVENT2_IMASK_MRXFIFOTRG_R = crate :: BitReader < INT_EVENT2_IMASK_MRXFIFOTRG_A > ; # [doc = "Master Receive FIFO Trigger Trigger when RX FIFO contains &amp;gt;= defined bytes\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_IMASK_MRXFIFOTRG_A { # [doc = "0: CLR"] INT_EVENT2_IMASK_MRXFIFOTRG_CLR = 0 , # [doc = "1: SET"] INT_EVENT2_IMASK_MRXFIFOTRG_SET = 1 , } impl From < INT_EVENT2_IMASK_MRXFIFOTRG_A > for bool { # [inline (always)] fn from (variant : INT_EVENT2_IMASK_MRXFIFOTRG_A) -> Self { variant as u8 != 0 } } impl INT_EVENT2_IMASK_MRXFIFOTRG_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT2_IMASK_MRXFIFOTRG_A { match self . bits { false => INT_EVENT2_IMASK_MRXFIFOTRG_A :: INT_EVENT2_IMASK_MRXFIFOTRG_CLR , true => INT_EVENT2_IMASK_MRXFIFOTRG_A :: INT_EVENT2_IMASK_MRXFIFOTRG_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event2_imask_mrxfifotrg_clr (& self) -> bool { * self == INT_EVENT2_IMASK_MRXFIFOTRG_A :: INT_EVENT2_IMASK_MRXFIFOTRG_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event2_imask_mrxfifotrg_set (& self) -> bool { * self == INT_EVENT2_IMASK_MRXFIFOTRG_A :: INT_EVENT2_IMASK_MRXFIFOTRG_SET } } # [doc = "Field `INT_EVENT2_IMASK_MRXFIFOTRG` writer - Master Receive FIFO Trigger Trigger when RX FIFO contains &amp;gt;= defined bytes"] pub type INT_EVENT2_IMASK_MRXFIFOTRG_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT2_IMASK_MRXFIFOTRG_A > ; impl < 'a , REG , const O : u8 > INT_EVENT2_IMASK_MRXFIFOTRG_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event2_imask_mrxfifotrg_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_IMASK_MRXFIFOTRG_A :: INT_EVENT2_IMASK_MRXFIFOTRG_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event2_imask_mrxfifotrg_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_IMASK_MRXFIFOTRG_A :: INT_EVENT2_IMASK_MRXFIFOTRG_SET) } } # [doc = "Field `INT_EVENT2_IMASK_MTXFIFOTRG` reader - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &amp;lt;= defined bytes"] pub type INT_EVENT2_IMASK_MTXFIFOTRG_R = crate :: BitReader < INT_EVENT2_IMASK_MTXFIFOTRG_A > ; # [doc = "Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &amp;lt;= defined bytes\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_IMASK_MTXFIFOTRG_A { # [doc = "0: CLR"] INT_EVENT2_IMASK_MTXFIFOTRG_CLR = 0 , # [doc = "1: SET"] INT_EVENT2_IMASK_MTXFIFOTRG_SET = 1 , } impl From < INT_EVENT2_IMASK_MTXFIFOTRG_A > for bool { # [inline (always)] fn from (variant : INT_EVENT2_IMASK_MTXFIFOTRG_A) -> Self { variant as u8 != 0 } } impl INT_EVENT2_IMASK_MTXFIFOTRG_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT2_IMASK_MTXFIFOTRG_A { match self . bits { false => INT_EVENT2_IMASK_MTXFIFOTRG_A :: INT_EVENT2_IMASK_MTXFIFOTRG_CLR , true => INT_EVENT2_IMASK_MTXFIFOTRG_A :: INT_EVENT2_IMASK_MTXFIFOTRG_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event2_imask_mtxfifotrg_clr (& self) -> bool { * self == INT_EVENT2_IMASK_MTXFIFOTRG_A :: INT_EVENT2_IMASK_MTXFIFOTRG_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event2_imask_mtxfifotrg_set (& self) -> bool { * self == INT_EVENT2_IMASK_MTXFIFOTRG_A :: INT_EVENT2_IMASK_MTXFIFOTRG_SET } } # [doc = "Field `INT_EVENT2_IMASK_MTXFIFOTRG` writer - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &amp;lt;= defined bytes"] pub type INT_EVENT2_IMASK_MTXFIFOTRG_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT2_IMASK_MTXFIFOTRG_A > ; impl < 'a , REG , const O : u8 > INT_EVENT2_IMASK_MTXFIFOTRG_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event2_imask_mtxfifotrg_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_IMASK_MTXFIFOTRG_A :: INT_EVENT2_IMASK_MTXFIFOTRG_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event2_imask_mtxfifotrg_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_IMASK_MTXFIFOTRG_A :: INT_EVENT2_IMASK_MTXFIFOTRG_SET) } } # [doc = "Field `INT_EVENT2_IMASK_SRXFIFOTRG` reader - Slave Receive FIFO Trigger"] pub type INT_EVENT2_IMASK_SRXFIFOTRG_R = crate :: BitReader < INT_EVENT2_IMASK_SRXFIFOTRG_A > ; # [doc = "Slave Receive FIFO Trigger\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_IMASK_SRXFIFOTRG_A { # [doc = "0: CLR"] INT_EVENT2_IMASK_SRXFIFOTRG_CLR = 0 , # [doc = "1: SET"] INT_EVENT2_IMASK_SRXFIFOTRG_SET = 1 , } impl From < INT_EVENT2_IMASK_SRXFIFOTRG_A > for bool { # [inline (always)] fn from (variant : INT_EVENT2_IMASK_SRXFIFOTRG_A) -> Self { variant as u8 != 0 } } impl INT_EVENT2_IMASK_SRXFIFOTRG_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT2_IMASK_SRXFIFOTRG_A { match self . bits { false => INT_EVENT2_IMASK_SRXFIFOTRG_A :: INT_EVENT2_IMASK_SRXFIFOTRG_CLR , true => INT_EVENT2_IMASK_SRXFIFOTRG_A :: INT_EVENT2_IMASK_SRXFIFOTRG_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event2_imask_srxfifotrg_clr (& self) -> bool { * self == INT_EVENT2_IMASK_SRXFIFOTRG_A :: INT_EVENT2_IMASK_SRXFIFOTRG_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event2_imask_srxfifotrg_set (& self) -> bool { * self == INT_EVENT2_IMASK_SRXFIFOTRG_A :: INT_EVENT2_IMASK_SRXFIFOTRG_SET } } # [doc = "Field `INT_EVENT2_IMASK_SRXFIFOTRG` writer - Slave Receive FIFO Trigger"] pub type INT_EVENT2_IMASK_SRXFIFOTRG_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT2_IMASK_SRXFIFOTRG_A > ; impl < 'a , REG , const O : u8 > INT_EVENT2_IMASK_SRXFIFOTRG_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event2_imask_srxfifotrg_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_IMASK_SRXFIFOTRG_A :: INT_EVENT2_IMASK_SRXFIFOTRG_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event2_imask_srxfifotrg_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_IMASK_SRXFIFOTRG_A :: INT_EVENT2_IMASK_SRXFIFOTRG_SET) } } # [doc = "Field `INT_EVENT2_IMASK_STXFIFOTRG` reader - Slave Transmit FIFO Trigger"] pub type INT_EVENT2_IMASK_STXFIFOTRG_R = crate :: BitReader < INT_EVENT2_IMASK_STXFIFOTRG_A > ; # [doc = "Slave Transmit FIFO Trigger\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_IMASK_STXFIFOTRG_A { # [doc = "0: CLR"] INT_EVENT2_IMASK_STXFIFOTRG_CLR = 0 , # [doc = "1: SET"] INT_EVENT2_IMASK_STXFIFOTRG_SET = 1 , } impl From < INT_EVENT2_IMASK_STXFIFOTRG_A > for bool { # [inline (always)] fn from (variant : INT_EVENT2_IMASK_STXFIFOTRG_A) -> Self { variant as u8 != 0 } } impl INT_EVENT2_IMASK_STXFIFOTRG_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT2_IMASK_STXFIFOTRG_A { match self . bits { false => INT_EVENT2_IMASK_STXFIFOTRG_A :: INT_EVENT2_IMASK_STXFIFOTRG_CLR , true => INT_EVENT2_IMASK_STXFIFOTRG_A :: INT_EVENT2_IMASK_STXFIFOTRG_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event2_imask_stxfifotrg_clr (& self) -> bool { * self == INT_EVENT2_IMASK_STXFIFOTRG_A :: INT_EVENT2_IMASK_STXFIFOTRG_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event2_imask_stxfifotrg_set (& self) -> bool { * self == INT_EVENT2_IMASK_STXFIFOTRG_A :: INT_EVENT2_IMASK_STXFIFOTRG_SET } } # [doc = "Field `INT_EVENT2_IMASK_STXFIFOTRG` writer - Slave Transmit FIFO Trigger"] pub type INT_EVENT2_IMASK_STXFIFOTRG_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT2_IMASK_STXFIFOTRG_A > ; impl < 'a , REG , const O : u8 > INT_EVENT2_IMASK_STXFIFOTRG_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event2_imask_stxfifotrg_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_IMASK_STXFIFOTRG_A :: INT_EVENT2_IMASK_STXFIFOTRG_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event2_imask_stxfifotrg_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_IMASK_STXFIFOTRG_A :: INT_EVENT2_IMASK_STXFIFOTRG_SET) } } impl R { # [doc = "Bit 0 - Master Receive FIFO Trigger Trigger when RX FIFO contains &amp;gt;= defined bytes"] # [inline (always)] pub fn int_event2_imask_mrxfifotrg (& self) -> INT_EVENT2_IMASK_MRXFIFOTRG_R { INT_EVENT2_IMASK_MRXFIFOTRG_R :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &amp;lt;= defined bytes"] # [inline (always)] pub fn int_event2_imask_mtxfifotrg (& self) -> INT_EVENT2_IMASK_MTXFIFOTRG_R { INT_EVENT2_IMASK_MTXFIFOTRG_R :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - Slave Receive FIFO Trigger"] # [inline (always)] pub fn int_event2_imask_srxfifotrg (& self) -> INT_EVENT2_IMASK_SRXFIFOTRG_R { INT_EVENT2_IMASK_SRXFIFOTRG_R :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 3 - Slave Transmit FIFO Trigger"] # [inline (always)] pub fn int_event2_imask_stxfifotrg (& self) -> INT_EVENT2_IMASK_STXFIFOTRG_R { INT_EVENT2_IMASK_STXFIFOTRG_R :: new (((self . bits >> 3) & 1) != 0) } } impl W { # [doc = "Bit 0 - Master Receive FIFO Trigger Trigger when RX FIFO contains &amp;gt;= defined bytes"] # [inline (always)] # [must_use] pub fn int_event2_imask_mrxfifotrg (& mut self) -> INT_EVENT2_IMASK_MRXFIFOTRG_W < INT_EVENT2_IMASK_SPEC , 0 > { INT_EVENT2_IMASK_MRXFIFOTRG_W :: new (self) } # [doc = "Bit 1 - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &amp;lt;= defined bytes"] # [inline (always)] # [must_use] pub fn int_event2_imask_mtxfifotrg (& mut self) -> INT_EVENT2_IMASK_MTXFIFOTRG_W < INT_EVENT2_IMASK_SPEC , 1 > { INT_EVENT2_IMASK_MTXFIFOTRG_W :: new (self) } # [doc = "Bit 2 - Slave Receive FIFO Trigger"] # [inline (always)] # [must_use] pub fn int_event2_imask_srxfifotrg (& mut self) -> INT_EVENT2_IMASK_SRXFIFOTRG_W < INT_EVENT2_IMASK_SPEC , 2 > { INT_EVENT2_IMASK_SRXFIFOTRG_W :: new (self) } # [doc = "Bit 3 - Slave Transmit FIFO Trigger"] # [inline (always)] # [must_use] pub fn int_event2_imask_stxfifotrg (& mut self) -> INT_EVENT2_IMASK_STXFIFOTRG_W < INT_EVENT2_IMASK_SPEC , 3 > { INT_EVENT2_IMASK_STXFIFOTRG_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_event2_imask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_event2_imask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct INT_EVENT2_IMASK_SPEC ; impl crate :: RegisterSpec for INT_EVENT2_IMASK_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`int_event2_imask::R`](R) reader structure"] impl crate :: Readable for INT_EVENT2_IMASK_SPEC { } # [doc = "`write(|w| ..)` method takes [`int_event2_imask::W`](W) writer structure"] impl crate :: Writable for INT_EVENT2_IMASK_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets INT_EVENT2_IMASK to value 0"] impl crate :: Resettable for INT_EVENT2_IMASK_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }