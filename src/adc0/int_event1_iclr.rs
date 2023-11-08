# [doc = "Register `INT_EVENT1_ICLR` writer"] pub type W = crate :: W < INT_EVENT1_ICLR_SPEC > ; # [doc = "Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_ICLR_HIGHIFG_AW { # [doc = "0: NO_EFFECT"] INT_EVENT1_ICLR_HIGHIFG_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT1_ICLR_HIGHIFG_CLR = 1 , } impl From < INT_EVENT1_ICLR_HIGHIFG_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT1_ICLR_HIGHIFG_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT1_ICLR_HIGHIFG` writer - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."] pub type INT_EVENT1_ICLR_HIGHIFG_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT1_ICLR_HIGHIFG_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT1_ICLR_HIGHIFG_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event1_iclr_highifg_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_ICLR_HIGHIFG_AW :: INT_EVENT1_ICLR_HIGHIFG_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event1_iclr_highifg_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_ICLR_HIGHIFG_AW :: INT_EVENT1_ICLR_HIGHIFG_CLR) } } # [doc = "Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_ICLR_LOWIFG_AW { # [doc = "0: NO_EFFECT"] INT_EVENT1_ICLR_LOWIFG_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT1_ICLR_LOWIFG_CLR = 1 , } impl From < INT_EVENT1_ICLR_LOWIFG_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT1_ICLR_LOWIFG_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT1_ICLR_LOWIFG` writer - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."] pub type INT_EVENT1_ICLR_LOWIFG_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT1_ICLR_LOWIFG_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT1_ICLR_LOWIFG_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event1_iclr_lowifg_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_ICLR_LOWIFG_AW :: INT_EVENT1_ICLR_LOWIFG_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event1_iclr_lowifg_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_ICLR_LOWIFG_AW :: INT_EVENT1_ICLR_LOWIFG_CLR) } } # [doc = "Mask INIFG in MIS_EX register.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_ICLR_INIFG_AW { # [doc = "0: NO_EFFECT"] INT_EVENT1_ICLR_INIFG_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT1_ICLR_INIFG_CLR = 1 , } impl From < INT_EVENT1_ICLR_INIFG_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT1_ICLR_INIFG_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT1_ICLR_INIFG` writer - Mask INIFG in MIS_EX register."] pub type INT_EVENT1_ICLR_INIFG_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT1_ICLR_INIFG_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT1_ICLR_INIFG_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event1_iclr_inifg_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_ICLR_INIFG_AW :: INT_EVENT1_ICLR_INIFG_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event1_iclr_inifg_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_ICLR_INIFG_AW :: INT_EVENT1_ICLR_INIFG_CLR) } } # [doc = "Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_ICLR_MEMRESIFG0_AW { # [doc = "0: NO_EFFECT"] INT_EVENT1_ICLR_MEMRESIFG0_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT1_ICLR_MEMRESIFG0_CLR = 1 , } impl From < INT_EVENT1_ICLR_MEMRESIFG0_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT1_ICLR_MEMRESIFG0_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT1_ICLR_MEMRESIFG0` writer - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] pub type INT_EVENT1_ICLR_MEMRESIFG0_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT1_ICLR_MEMRESIFG0_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT1_ICLR_MEMRESIFG0_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event1_iclr_memresifg0_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_ICLR_MEMRESIFG0_AW :: INT_EVENT1_ICLR_MEMRESIFG0_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event1_iclr_memresifg0_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_ICLR_MEMRESIFG0_AW :: INT_EVENT1_ICLR_MEMRESIFG0_CLR) } } impl W { # [doc = "Bit 2 - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."] # [inline (always)] # [must_use] pub fn int_event1_iclr_highifg (& mut self) -> INT_EVENT1_ICLR_HIGHIFG_W < INT_EVENT1_ICLR_SPEC , 2 > { INT_EVENT1_ICLR_HIGHIFG_W :: new (self) } # [doc = "Bit 3 - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."] # [inline (always)] # [must_use] pub fn int_event1_iclr_lowifg (& mut self) -> INT_EVENT1_ICLR_LOWIFG_W < INT_EVENT1_ICLR_SPEC , 3 > { INT_EVENT1_ICLR_LOWIFG_W :: new (self) } # [doc = "Bit 4 - Mask INIFG in MIS_EX register."] # [inline (always)] # [must_use] pub fn int_event1_iclr_inifg (& mut self) -> INT_EVENT1_ICLR_INIFG_W < INT_EVENT1_ICLR_SPEC , 4 > { INT_EVENT1_ICLR_INIFG_W :: new (self) } # [doc = "Bit 8 - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"] # [inline (always)] # [must_use] pub fn int_event1_iclr_memresifg0 (& mut self) -> INT_EVENT1_ICLR_MEMRESIFG0_W < INT_EVENT1_ICLR_SPEC , 8 > { INT_EVENT1_ICLR_MEMRESIFG0_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Interrupt clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_event1_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct INT_EVENT1_ICLR_SPEC ; impl crate :: RegisterSpec for INT_EVENT1_ICLR_SPEC { type Ux = u32 ; } # [doc = "`write(|w| ..)` method takes [`int_event1_iclr::W`](W) writer structure"] impl crate :: Writable for INT_EVENT1_ICLR_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets INT_EVENT1_ICLR to value 0"] impl crate :: Resettable for INT_EVENT1_ICLR_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }