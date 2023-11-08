# [doc = "Register `INT_EVENT2_IMASK` reader"] pub type R = crate :: R < INT_EVENT2_IMASK_SPEC > ; # [doc = "Register `INT_EVENT2_IMASK` writer"] pub type W = crate :: W < INT_EVENT2_IMASK_SPEC > ; # [doc = "Field `INT_EVENT2_IMASK_MEMRESIFG0` reader - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] pub type INT_EVENT2_IMASK_MEMRESIFG0_R = crate :: BitReader < INT_EVENT2_IMASK_MEMRESIFG0_A > ; # [doc = "Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_IMASK_MEMRESIFG0_A { # [doc = "0: CLR"] INT_EVENT2_IMASK_MEMRESIFG0_CLR = 0 , # [doc = "1: SET"] INT_EVENT2_IMASK_MEMRESIFG0_SET = 1 , } impl From < INT_EVENT2_IMASK_MEMRESIFG0_A > for bool { # [inline (always)] fn from (variant : INT_EVENT2_IMASK_MEMRESIFG0_A) -> Self { variant as u8 != 0 } } impl INT_EVENT2_IMASK_MEMRESIFG0_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT2_IMASK_MEMRESIFG0_A { match self . bits { false => INT_EVENT2_IMASK_MEMRESIFG0_A :: INT_EVENT2_IMASK_MEMRESIFG0_CLR , true => INT_EVENT2_IMASK_MEMRESIFG0_A :: INT_EVENT2_IMASK_MEMRESIFG0_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event2_imask_memresifg0_clr (& self) -> bool { * self == INT_EVENT2_IMASK_MEMRESIFG0_A :: INT_EVENT2_IMASK_MEMRESIFG0_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event2_imask_memresifg0_set (& self) -> bool { * self == INT_EVENT2_IMASK_MEMRESIFG0_A :: INT_EVENT2_IMASK_MEMRESIFG0_SET } } # [doc = "Field `INT_EVENT2_IMASK_MEMRESIFG0` writer - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] pub type INT_EVENT2_IMASK_MEMRESIFG0_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT2_IMASK_MEMRESIFG0_A > ; impl < 'a , REG , const O : u8 > INT_EVENT2_IMASK_MEMRESIFG0_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event2_imask_memresifg0_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_IMASK_MEMRESIFG0_A :: INT_EVENT2_IMASK_MEMRESIFG0_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event2_imask_memresifg0_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_IMASK_MEMRESIFG0_A :: INT_EVENT2_IMASK_MEMRESIFG0_SET) } } # [doc = "Field `INT_EVENT2_IMASK_MEMRESIFG1` reader - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] pub type INT_EVENT2_IMASK_MEMRESIFG1_R = crate :: BitReader < INT_EVENT2_IMASK_MEMRESIFG1_A > ; # [doc = "Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_IMASK_MEMRESIFG1_A { # [doc = "0: CLR"] INT_EVENT2_IMASK_MEMRESIFG1_CLR = 0 , # [doc = "1: SET"] INT_EVENT2_IMASK_MEMRESIFG1_SET = 1 , } impl From < INT_EVENT2_IMASK_MEMRESIFG1_A > for bool { # [inline (always)] fn from (variant : INT_EVENT2_IMASK_MEMRESIFG1_A) -> Self { variant as u8 != 0 } } impl INT_EVENT2_IMASK_MEMRESIFG1_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT2_IMASK_MEMRESIFG1_A { match self . bits { false => INT_EVENT2_IMASK_MEMRESIFG1_A :: INT_EVENT2_IMASK_MEMRESIFG1_CLR , true => INT_EVENT2_IMASK_MEMRESIFG1_A :: INT_EVENT2_IMASK_MEMRESIFG1_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event2_imask_memresifg1_clr (& self) -> bool { * self == INT_EVENT2_IMASK_MEMRESIFG1_A :: INT_EVENT2_IMASK_MEMRESIFG1_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event2_imask_memresifg1_set (& self) -> bool { * self == INT_EVENT2_IMASK_MEMRESIFG1_A :: INT_EVENT2_IMASK_MEMRESIFG1_SET } } # [doc = "Field `INT_EVENT2_IMASK_MEMRESIFG1` writer - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] pub type INT_EVENT2_IMASK_MEMRESIFG1_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT2_IMASK_MEMRESIFG1_A > ; impl < 'a , REG , const O : u8 > INT_EVENT2_IMASK_MEMRESIFG1_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event2_imask_memresifg1_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_IMASK_MEMRESIFG1_A :: INT_EVENT2_IMASK_MEMRESIFG1_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event2_imask_memresifg1_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_IMASK_MEMRESIFG1_A :: INT_EVENT2_IMASK_MEMRESIFG1_SET) } } # [doc = "Field `INT_EVENT2_IMASK_MEMRESIFG2` reader - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] pub type INT_EVENT2_IMASK_MEMRESIFG2_R = crate :: BitReader < INT_EVENT2_IMASK_MEMRESIFG2_A > ; # [doc = "Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_IMASK_MEMRESIFG2_A { # [doc = "0: CLR"] INT_EVENT2_IMASK_MEMRESIFG2_CLR = 0 , # [doc = "1: SET"] INT_EVENT2_IMASK_MEMRESIFG2_SET = 1 , } impl From < INT_EVENT2_IMASK_MEMRESIFG2_A > for bool { # [inline (always)] fn from (variant : INT_EVENT2_IMASK_MEMRESIFG2_A) -> Self { variant as u8 != 0 } } impl INT_EVENT2_IMASK_MEMRESIFG2_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT2_IMASK_MEMRESIFG2_A { match self . bits { false => INT_EVENT2_IMASK_MEMRESIFG2_A :: INT_EVENT2_IMASK_MEMRESIFG2_CLR , true => INT_EVENT2_IMASK_MEMRESIFG2_A :: INT_EVENT2_IMASK_MEMRESIFG2_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event2_imask_memresifg2_clr (& self) -> bool { * self == INT_EVENT2_IMASK_MEMRESIFG2_A :: INT_EVENT2_IMASK_MEMRESIFG2_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event2_imask_memresifg2_set (& self) -> bool { * self == INT_EVENT2_IMASK_MEMRESIFG2_A :: INT_EVENT2_IMASK_MEMRESIFG2_SET } } # [doc = "Field `INT_EVENT2_IMASK_MEMRESIFG2` writer - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] pub type INT_EVENT2_IMASK_MEMRESIFG2_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT2_IMASK_MEMRESIFG2_A > ; impl < 'a , REG , const O : u8 > INT_EVENT2_IMASK_MEMRESIFG2_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event2_imask_memresifg2_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_IMASK_MEMRESIFG2_A :: INT_EVENT2_IMASK_MEMRESIFG2_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event2_imask_memresifg2_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_IMASK_MEMRESIFG2_A :: INT_EVENT2_IMASK_MEMRESIFG2_SET) } } # [doc = "Field `INT_EVENT2_IMASK_MEMRESIFG3` reader - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] pub type INT_EVENT2_IMASK_MEMRESIFG3_R = crate :: BitReader < INT_EVENT2_IMASK_MEMRESIFG3_A > ; # [doc = "Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_IMASK_MEMRESIFG3_A { # [doc = "0: CLR"] INT_EVENT2_IMASK_MEMRESIFG3_CLR = 0 , # [doc = "1: SET"] INT_EVENT2_IMASK_MEMRESIFG3_SET = 1 , } impl From < INT_EVENT2_IMASK_MEMRESIFG3_A > for bool { # [inline (always)] fn from (variant : INT_EVENT2_IMASK_MEMRESIFG3_A) -> Self { variant as u8 != 0 } } impl INT_EVENT2_IMASK_MEMRESIFG3_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT2_IMASK_MEMRESIFG3_A { match self . bits { false => INT_EVENT2_IMASK_MEMRESIFG3_A :: INT_EVENT2_IMASK_MEMRESIFG3_CLR , true => INT_EVENT2_IMASK_MEMRESIFG3_A :: INT_EVENT2_IMASK_MEMRESIFG3_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event2_imask_memresifg3_clr (& self) -> bool { * self == INT_EVENT2_IMASK_MEMRESIFG3_A :: INT_EVENT2_IMASK_MEMRESIFG3_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event2_imask_memresifg3_set (& self) -> bool { * self == INT_EVENT2_IMASK_MEMRESIFG3_A :: INT_EVENT2_IMASK_MEMRESIFG3_SET } } # [doc = "Field `INT_EVENT2_IMASK_MEMRESIFG3` writer - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] pub type INT_EVENT2_IMASK_MEMRESIFG3_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT2_IMASK_MEMRESIFG3_A > ; impl < 'a , REG , const O : u8 > INT_EVENT2_IMASK_MEMRESIFG3_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event2_imask_memresifg3_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_IMASK_MEMRESIFG3_A :: INT_EVENT2_IMASK_MEMRESIFG3_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event2_imask_memresifg3_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_IMASK_MEMRESIFG3_A :: INT_EVENT2_IMASK_MEMRESIFG3_SET) } } impl R { # [doc = "Bit 8 - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] # [inline (always)] pub fn int_event2_imask_memresifg0 (& self) -> INT_EVENT2_IMASK_MEMRESIFG0_R { INT_EVENT2_IMASK_MEMRESIFG0_R :: new (((self . bits >> 8) & 1) != 0) } # [doc = "Bit 9 - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] # [inline (always)] pub fn int_event2_imask_memresifg1 (& self) -> INT_EVENT2_IMASK_MEMRESIFG1_R { INT_EVENT2_IMASK_MEMRESIFG1_R :: new (((self . bits >> 9) & 1) != 0) } # [doc = "Bit 10 - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] # [inline (always)] pub fn int_event2_imask_memresifg2 (& self) -> INT_EVENT2_IMASK_MEMRESIFG2_R { INT_EVENT2_IMASK_MEMRESIFG2_R :: new (((self . bits >> 10) & 1) != 0) } # [doc = "Bit 11 - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] # [inline (always)] pub fn int_event2_imask_memresifg3 (& self) -> INT_EVENT2_IMASK_MEMRESIFG3_R { INT_EVENT2_IMASK_MEMRESIFG3_R :: new (((self . bits >> 11) & 1) != 0) } } impl W { # [doc = "Bit 8 - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] # [inline (always)] # [must_use] pub fn int_event2_imask_memresifg0 (& mut self) -> INT_EVENT2_IMASK_MEMRESIFG0_W < INT_EVENT2_IMASK_SPEC , 8 > { INT_EVENT2_IMASK_MEMRESIFG0_W :: new (self) } # [doc = "Bit 9 - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] # [inline (always)] # [must_use] pub fn int_event2_imask_memresifg1 (& mut self) -> INT_EVENT2_IMASK_MEMRESIFG1_W < INT_EVENT2_IMASK_SPEC , 9 > { INT_EVENT2_IMASK_MEMRESIFG1_W :: new (self) } # [doc = "Bit 10 - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] # [inline (always)] # [must_use] pub fn int_event2_imask_memresifg2 (& mut self) -> INT_EVENT2_IMASK_MEMRESIFG2_W < INT_EVENT2_IMASK_SPEC , 10 > { INT_EVENT2_IMASK_MEMRESIFG2_W :: new (self) } # [doc = "Bit 11 - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] # [inline (always)] # [must_use] pub fn int_event2_imask_memresifg3 (& mut self) -> INT_EVENT2_IMASK_MEMRESIFG3_W < INT_EVENT2_IMASK_SPEC , 11 > { INT_EVENT2_IMASK_MEMRESIFG3_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Interrupt mask extension\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_event2_imask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_event2_imask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct INT_EVENT2_IMASK_SPEC ; impl crate :: RegisterSpec for INT_EVENT2_IMASK_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`int_event2_imask::R`](R) reader structure"] impl crate :: Readable for INT_EVENT2_IMASK_SPEC { } # [doc = "`write(|w| ..)` method takes [`int_event2_imask::W`](W) writer structure"] impl crate :: Writable for INT_EVENT2_IMASK_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets INT_EVENT2_IMASK to value 0"] impl crate :: Resettable for INT_EVENT2_IMASK_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }