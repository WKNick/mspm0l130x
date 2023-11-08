# [doc = "Register `DMASZ` reader"] pub type R = crate :: R < DMASZ_SPEC > ; # [doc = "Register `DMASZ` writer"] pub type W = crate :: W < DMASZ_SPEC > ; # [doc = "Field `DMASZ_SIZE` reader - DMA Channel Size in number of transfers"] pub type DMASZ_SIZE_R = crate :: FieldReader < u16 > ; # [doc = "Field `DMASZ_SIZE` writer - DMA Channel Size in number of transfers"] pub type DMASZ_SIZE_W < 'a , REG , const O : u8 > = crate :: FieldWriter < 'a , REG , 16 , O , u16 > ; impl R { # [doc = "Bits 0:15 - DMA Channel Size in number of transfers"] # [inline (always)] pub fn dmasz_size (& self) -> DMASZ_SIZE_R { DMASZ_SIZE_R :: new ((self . bits & 0xffff) as u16) } } impl W { # [doc = "Bits 0:15 - DMA Channel Size in number of transfers"] # [inline (always)] # [must_use] pub fn dmasz_size (& mut self) -> DMASZ_SIZE_W < DMASZ_SPEC , 0 > { DMASZ_SIZE_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "DMA Channel Size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmasz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmasz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct DMASZ_SPEC ; impl crate :: RegisterSpec for DMASZ_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`dmasz::R`](R) reader structure"] impl crate :: Readable for DMASZ_SPEC { } # [doc = "`write(|w| ..)` method takes [`dmasz::W`](W) writer structure"] impl crate :: Writable for DMASZ_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets DMASZ to value 0"] impl crate :: Resettable for DMASZ_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }