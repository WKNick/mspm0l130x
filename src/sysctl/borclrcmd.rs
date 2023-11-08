# [doc = "Register `BORCLRCMD` writer"] pub type W = crate :: W < BORCLRCMD_SPEC > ; # [doc = "GO clears any prior BOR violation status indications and attempts to change the active BOR mode to that specified in the LEVEL field of the BORTHRESHOLD register.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum BORCLRCMD_GO_AW { # [doc = "1: TRUE"] BORCLRCMD_GO_TRUE = 1 , } impl From < BORCLRCMD_GO_AW > for bool { # [inline (always)] fn from (variant : BORCLRCMD_GO_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `BORCLRCMD_GO` writer - GO clears any prior BOR violation status indications and attempts to change the active BOR mode to that specified in the LEVEL field of the BORTHRESHOLD register."] pub type BORCLRCMD_GO_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , BORCLRCMD_GO_AW > ; impl < 'a , REG , const O : u8 > BORCLRCMD_GO_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "TRUE"] # [inline (always)] pub fn borclrcmd_go_true (self) -> & 'a mut crate :: W < REG > { self . variant (BORCLRCMD_GO_AW :: BORCLRCMD_GO_TRUE) } } # [doc = "The key value of C7h (199) must be written to KEY together with GO to trigger the clear and BOR threshold change.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum BORCLRCMD_KEY_AW { # [doc = "199: VALUE"] BORCLRCMD_KEY_VALUE = 199 , } impl From < BORCLRCMD_KEY_AW > for u8 { # [inline (always)] fn from (variant : BORCLRCMD_KEY_AW) -> Self { variant as _ } } impl crate :: FieldSpec for BORCLRCMD_KEY_AW { type Ux = u8 ; } # [doc = "Field `BORCLRCMD_KEY` writer - The key value of C7h (199) must be written to KEY together with GO to trigger the clear and BOR threshold change."] pub type BORCLRCMD_KEY_W < 'a , REG , const O : u8 > = crate :: FieldWriter < 'a , REG , 8 , O , BORCLRCMD_KEY_AW > ; impl < 'a , REG , const O : u8 > BORCLRCMD_KEY_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "VALUE"] # [inline (always)] pub fn borclrcmd_key_value (self) -> & 'a mut crate :: W < REG > { self . variant (BORCLRCMD_KEY_AW :: BORCLRCMD_KEY_VALUE) } } impl W { # [doc = "Bit 0 - GO clears any prior BOR violation status indications and attempts to change the active BOR mode to that specified in the LEVEL field of the BORTHRESHOLD register."] # [inline (always)] # [must_use] pub fn borclrcmd_go (& mut self) -> BORCLRCMD_GO_W < BORCLRCMD_SPEC , 0 > { BORCLRCMD_GO_W :: new (self) } # [doc = "Bits 24:31 - The key value of C7h (199) must be written to KEY together with GO to trigger the clear and BOR threshold change."] # [inline (always)] # [must_use] pub fn borclrcmd_key (& mut self) -> BORCLRCMD_KEY_W < BORCLRCMD_SPEC , 24 > { BORCLRCMD_KEY_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Set the BOR threshold\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`borclrcmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct BORCLRCMD_SPEC ; impl crate :: RegisterSpec for BORCLRCMD_SPEC { type Ux = u32 ; } # [doc = "`write(|w| ..)` method takes [`borclrcmd::W`](W) writer structure"] impl crate :: Writable for BORCLRCMD_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets BORCLRCMD to value 0"] impl crate :: Resettable for BORCLRCMD_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }