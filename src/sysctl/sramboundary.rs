# [doc = "Register `SRAMBOUNDARY` reader"] pub type R = crate :: R < SRAMBOUNDARY_SPEC > ; # [doc = "Register `SRAMBOUNDARY` writer"] pub type W = crate :: W < SRAMBOUNDARY_SPEC > ; # [doc = "Field `SRAMBOUNDARY_ADDR` reader - SRAM boundary configuration. The value configured into this acts such that: SRAM accesses to addresses less than or equal value will be RW only. SRAM accesses to addresses greater than value will be RX only. Value of 0 is not valid (system will have no stack). If set to 0, the system acts as if the entire SRAM is RWX. Any non-zero value can be configured, including a value = SRAM size."] pub type SRAMBOUNDARY_ADDR_R = crate :: FieldReader < u16 > ; # [doc = "Field `SRAMBOUNDARY_ADDR` writer - SRAM boundary configuration. The value configured into this acts such that: SRAM accesses to addresses less than or equal value will be RW only. SRAM accesses to addresses greater than value will be RX only. Value of 0 is not valid (system will have no stack). If set to 0, the system acts as if the entire SRAM is RWX. Any non-zero value can be configured, including a value = SRAM size."] pub type SRAMBOUNDARY_ADDR_W < 'a , REG , const O : u8 > = crate :: FieldWriter < 'a , REG , 15 , O , u16 > ; impl R { # [doc = "Bits 5:19 - SRAM boundary configuration. The value configured into this acts such that: SRAM accesses to addresses less than or equal value will be RW only. SRAM accesses to addresses greater than value will be RX only. Value of 0 is not valid (system will have no stack). If set to 0, the system acts as if the entire SRAM is RWX. Any non-zero value can be configured, including a value = SRAM size."] # [inline (always)] pub fn sramboundary_addr (& self) -> SRAMBOUNDARY_ADDR_R { SRAMBOUNDARY_ADDR_R :: new (((self . bits >> 5) & 0x7fff) as u16) } } impl W { # [doc = "Bits 5:19 - SRAM boundary configuration. The value configured into this acts such that: SRAM accesses to addresses less than or equal value will be RW only. SRAM accesses to addresses greater than value will be RX only. Value of 0 is not valid (system will have no stack). If set to 0, the system acts as if the entire SRAM is RWX. Any non-zero value can be configured, including a value = SRAM size."] # [inline (always)] # [must_use] pub fn sramboundary_addr (& mut self) -> SRAMBOUNDARY_ADDR_W < SRAMBOUNDARY_SPEC , 5 > { SRAMBOUNDARY_ADDR_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "SRAM Write Boundary\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sramboundary::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sramboundary::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct SRAMBOUNDARY_SPEC ; impl crate :: RegisterSpec for SRAMBOUNDARY_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`sramboundary::R`](R) reader structure"] impl crate :: Readable for SRAMBOUNDARY_SPEC { } # [doc = "`write(|w| ..)` method takes [`sramboundary::W`](W) writer structure"] impl crate :: Writable for SRAMBOUNDARY_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets SRAMBOUNDARY to value 0"] impl crate :: Resettable for SRAMBOUNDARY_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }