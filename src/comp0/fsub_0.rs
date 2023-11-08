# [doc = "Register `FSUB_0` reader"] pub type R = crate :: R < FSUB_0_SPEC > ; # [doc = "Register `FSUB_0` writer"] pub type W = crate :: W < FSUB_0_SPEC > ; # [doc = "Field `FSUB_0_CHANID` reader - 0 = disconnected. 1-15 = connected to channelID = CHANID."] pub type FSUB_0_CHANID_R = crate :: FieldReader < FSUB_0_CHANID_A > ; # [doc = "0 = disconnected. 1-15 = connected to channelID = CHANID.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum FSUB_0_CHANID_A { # [doc = "0: UNCONNECTED"] FSUB_0_CHANID_UNCONNECTED = 0 , } impl From < FSUB_0_CHANID_A > for u8 { # [inline (always)] fn from (variant : FSUB_0_CHANID_A) -> Self { variant as _ } } impl crate :: FieldSpec for FSUB_0_CHANID_A { type Ux = u8 ; } impl FSUB_0_CHANID_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < FSUB_0_CHANID_A > { match self . bits { 0 => Some (FSUB_0_CHANID_A :: FSUB_0_CHANID_UNCONNECTED) , _ => None , } } # [doc = "UNCONNECTED"] # [inline (always)] pub fn is_fsub_0_chanid_unconnected (& self) -> bool { * self == FSUB_0_CHANID_A :: FSUB_0_CHANID_UNCONNECTED } } # [doc = "Field `FSUB_0_CHANID` writer - 0 = disconnected. 1-15 = connected to channelID = CHANID."] pub type FSUB_0_CHANID_W < 'a , REG , const O : u8 > = crate :: FieldWriter < 'a , REG , 2 , O , FSUB_0_CHANID_A > ; impl < 'a , REG , const O : u8 > FSUB_0_CHANID_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "UNCONNECTED"] # [inline (always)] pub fn fsub_0_chanid_unconnected (self) -> & 'a mut crate :: W < REG > { self . variant (FSUB_0_CHANID_A :: FSUB_0_CHANID_UNCONNECTED) } } impl R { # [doc = "Bits 0:1 - 0 = disconnected. 1-15 = connected to channelID = CHANID."] # [inline (always)] pub fn fsub_0_chanid (& self) -> FSUB_0_CHANID_R { FSUB_0_CHANID_R :: new ((self . bits & 3) as u8) } } impl W { # [doc = "Bits 0:1 - 0 = disconnected. 1-15 = connected to channelID = CHANID."] # [inline (always)] # [must_use] pub fn fsub_0_chanid (& mut self) -> FSUB_0_CHANID_W < FSUB_0_SPEC , 0 > { FSUB_0_CHANID_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Subscriber Port 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsub_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsub_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct FSUB_0_SPEC ; impl crate :: RegisterSpec for FSUB_0_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`fsub_0::R`](R) reader structure"] impl crate :: Readable for FSUB_0_SPEC { } # [doc = "`write(|w| ..)` method takes [`fsub_0::W`](W) writer structure"] impl crate :: Writable for FSUB_0_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets FSUB_0 to value 0"] impl crate :: Resettable for FSUB_0_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }