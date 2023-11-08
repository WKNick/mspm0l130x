# [doc = "Register `FCCCMD` writer"] pub type W = crate :: W < FCCCMD_SPEC > ; # [doc = "Set GO to start a capture with the frequency clock counter (FCC).\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum FCCCMD_GO_AW { # [doc = "1: TRUE"] FCCCMD_GO_TRUE = 1 , } impl From < FCCCMD_GO_AW > for bool { # [inline (always)] fn from (variant : FCCCMD_GO_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `FCCCMD_GO` writer - Set GO to start a capture with the frequency clock counter (FCC)."] pub type FCCCMD_GO_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , FCCCMD_GO_AW > ; impl < 'a , REG , const O : u8 > FCCCMD_GO_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "TRUE"] # [inline (always)] pub fn fcccmd_go_true (self) -> & 'a mut crate :: W < REG > { self . variant (FCCCMD_GO_AW :: FCCCMD_GO_TRUE) } } # [doc = "The key value 0Eh (14) must be written with GO to start a capture.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum FCCCMD_KEY_AW { # [doc = "14: VALUE"] FCCCMD_KEY_VALUE = 14 , } impl From < FCCCMD_KEY_AW > for u8 { # [inline (always)] fn from (variant : FCCCMD_KEY_AW) -> Self { variant as _ } } impl crate :: FieldSpec for FCCCMD_KEY_AW { type Ux = u8 ; } # [doc = "Field `FCCCMD_KEY` writer - The key value 0Eh (14) must be written with GO to start a capture."] pub type FCCCMD_KEY_W < 'a , REG , const O : u8 > = crate :: FieldWriter < 'a , REG , 8 , O , FCCCMD_KEY_AW > ; impl < 'a , REG , const O : u8 > FCCCMD_KEY_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "VALUE"] # [inline (always)] pub fn fcccmd_key_value (self) -> & 'a mut crate :: W < REG > { self . variant (FCCCMD_KEY_AW :: FCCCMD_KEY_VALUE) } } impl W { # [doc = "Bit 0 - Set GO to start a capture with the frequency clock counter (FCC)."] # [inline (always)] # [must_use] pub fn fcccmd_go (& mut self) -> FCCCMD_GO_W < FCCCMD_SPEC , 0 > { FCCCMD_GO_W :: new (self) } # [doc = "Bits 24:31 - The key value 0Eh (14) must be written with GO to start a capture."] # [inline (always)] # [must_use] pub fn fcccmd_key (& mut self) -> FCCCMD_KEY_W < FCCCMD_SPEC , 24 > { FCCCMD_KEY_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Frequency clock counter start capture\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcccmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct FCCCMD_SPEC ; impl crate :: RegisterSpec for FCCCMD_SPEC { type Ux = u32 ; } # [doc = "`write(|w| ..)` method takes [`fcccmd::W`](W) writer structure"] impl crate :: Writable for FCCCMD_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets FCCCMD to value 0"] impl crate :: Resettable for FCCCMD_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }