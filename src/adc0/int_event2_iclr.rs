# [doc = "Register `INT_EVENT2_ICLR` writer"] pub type W = crate :: W < INT_EVENT2_ICLR_SPEC > ; # [doc = "Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_ICLR_MEMRESIFG0_AW { # [doc = "0: NO_EFFECT"] INT_EVENT2_ICLR_MEMRESIFG0_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT2_ICLR_MEMRESIFG0_CLR = 1 , } impl From < INT_EVENT2_ICLR_MEMRESIFG0_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT2_ICLR_MEMRESIFG0_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT2_ICLR_MEMRESIFG0` writer - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] pub type INT_EVENT2_ICLR_MEMRESIFG0_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT2_ICLR_MEMRESIFG0_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT2_ICLR_MEMRESIFG0_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event2_iclr_memresifg0_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_MEMRESIFG0_AW :: INT_EVENT2_ICLR_MEMRESIFG0_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event2_iclr_memresifg0_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_MEMRESIFG0_AW :: INT_EVENT2_ICLR_MEMRESIFG0_CLR) } } # [doc = "Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_ICLR_MEMRESIFG1_AW { # [doc = "0: NO_EFFECT"] INT_EVENT2_ICLR_MEMRESIFG1_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT2_ICLR_MEMRESIFG1_CLR = 1 , } impl From < INT_EVENT2_ICLR_MEMRESIFG1_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT2_ICLR_MEMRESIFG1_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT2_ICLR_MEMRESIFG1` writer - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] pub type INT_EVENT2_ICLR_MEMRESIFG1_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT2_ICLR_MEMRESIFG1_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT2_ICLR_MEMRESIFG1_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event2_iclr_memresifg1_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_MEMRESIFG1_AW :: INT_EVENT2_ICLR_MEMRESIFG1_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event2_iclr_memresifg1_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_MEMRESIFG1_AW :: INT_EVENT2_ICLR_MEMRESIFG1_CLR) } } # [doc = "Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_ICLR_MEMRESIFG2_AW { # [doc = "0: NO_EFFECT"] INT_EVENT2_ICLR_MEMRESIFG2_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT2_ICLR_MEMRESIFG2_CLR = 1 , } impl From < INT_EVENT2_ICLR_MEMRESIFG2_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT2_ICLR_MEMRESIFG2_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT2_ICLR_MEMRESIFG2` writer - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] pub type INT_EVENT2_ICLR_MEMRESIFG2_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT2_ICLR_MEMRESIFG2_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT2_ICLR_MEMRESIFG2_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event2_iclr_memresifg2_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_MEMRESIFG2_AW :: INT_EVENT2_ICLR_MEMRESIFG2_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event2_iclr_memresifg2_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_MEMRESIFG2_AW :: INT_EVENT2_ICLR_MEMRESIFG2_CLR) } } # [doc = "Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_ICLR_MEMRESIFG3_AW { # [doc = "0: NO_EFFECT"] INT_EVENT2_ICLR_MEMRESIFG3_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT2_ICLR_MEMRESIFG3_CLR = 1 , } impl From < INT_EVENT2_ICLR_MEMRESIFG3_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT2_ICLR_MEMRESIFG3_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT2_ICLR_MEMRESIFG3` writer - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] pub type INT_EVENT2_ICLR_MEMRESIFG3_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT2_ICLR_MEMRESIFG3_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT2_ICLR_MEMRESIFG3_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event2_iclr_memresifg3_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_MEMRESIFG3_AW :: INT_EVENT2_ICLR_MEMRESIFG3_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event2_iclr_memresifg3_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_MEMRESIFG3_AW :: INT_EVENT2_ICLR_MEMRESIFG3_CLR) } } impl W { # [doc = "Bit 8 - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] # [inline (always)] # [must_use] pub fn int_event2_iclr_memresifg0 (& mut self) -> INT_EVENT2_ICLR_MEMRESIFG0_W < INT_EVENT2_ICLR_SPEC , 8 > { INT_EVENT2_ICLR_MEMRESIFG0_W :: new (self) } # [doc = "Bit 9 - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] # [inline (always)] # [must_use] pub fn int_event2_iclr_memresifg1 (& mut self) -> INT_EVENT2_ICLR_MEMRESIFG1_W < INT_EVENT2_ICLR_SPEC , 9 > { INT_EVENT2_ICLR_MEMRESIFG1_W :: new (self) } # [doc = "Bit 10 - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] # [inline (always)] # [must_use] pub fn int_event2_iclr_memresifg2 (& mut self) -> INT_EVENT2_ICLR_MEMRESIFG2_W < INT_EVENT2_ICLR_SPEC , 10 > { INT_EVENT2_ICLR_MEMRESIFG2_W :: new (self) } # [doc = "Bit 11 - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] # [inline (always)] # [must_use] pub fn int_event2_iclr_memresifg3 (& mut self) -> INT_EVENT2_ICLR_MEMRESIFG3_W < INT_EVENT2_ICLR_SPEC , 11 > { INT_EVENT2_ICLR_MEMRESIFG3_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Interrupt clear extension\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_event2_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct INT_EVENT2_ICLR_SPEC ; impl crate :: RegisterSpec for INT_EVENT2_ICLR_SPEC { type Ux = u32 ; } # [doc = "`write(|w| ..)` method takes [`int_event2_iclr::W`](W) writer structure"] impl crate :: Writable for INT_EVENT2_ICLR_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets INT_EVENT2_ICLR to value 0"] impl crate :: Resettable for INT_EVENT2_ICLR_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }