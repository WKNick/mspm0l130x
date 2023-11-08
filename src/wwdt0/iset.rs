# [doc = "Register `ISET` writer"] pub type W = crate :: W < ISET_SPEC > ; # [doc = "Interval Timer Interrupt.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum ISET_INTTIM_AW { # [doc = "0: NO_EFFECT"] ISET_INTTIM_NO_EFFECT = 0 , # [doc = "1: SET"] ISET_INTTIM_SET = 1 , } impl From < ISET_INTTIM_AW > for bool { # [inline (always)] fn from (variant : ISET_INTTIM_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `ISET_INTTIM` writer - Interval Timer Interrupt."] pub type ISET_INTTIM_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , ISET_INTTIM_AW > ; impl < 'a , REG , const O : u8 > ISET_INTTIM_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn iset_inttim_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (ISET_INTTIM_AW :: ISET_INTTIM_NO_EFFECT) } # [doc = "SET"] # [inline (always)] pub fn iset_inttim_set (self) -> & 'a mut crate :: W < REG > { self . variant (ISET_INTTIM_AW :: ISET_INTTIM_SET) } } impl W { # [doc = "Bit 0 - Interval Timer Interrupt."] # [inline (always)] # [must_use] pub fn iset_inttim (& mut self) -> ISET_INTTIM_W < ISET_SPEC , 0 > { ISET_INTTIM_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Interrupt set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct ISET_SPEC ; impl crate :: RegisterSpec for ISET_SPEC { type Ux = u32 ; } # [doc = "`write(|w| ..)` method takes [`iset::W`](W) writer structure"] impl crate :: Writable for ISET_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets ISET to value 0"] impl crate :: Resettable for ISET_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }