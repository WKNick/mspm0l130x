# [doc = "Register `CTL2` reader"] pub type R = crate :: R < CTL2_SPEC > ; # [doc = "Register `CTL2` writer"] pub type W = crate :: W < CTL2_SPEC > ; # [doc = "Field `CTL2_REFMODE` reader - This bit requests ULP_REF bandgap operation in static mode or sampled mode. The local reference buffer and 8-bit DAC inside comparator module are also configured accordingly. Static mode operation offers higher accuracy but consumes higher current. Sampled mode operation consumes lower current but with relaxed reference voltage accuracy. Comparator requests for reference voltage from ULP_REF only when REFLVL &amp;gt; 0."] pub type CTL2_REFMODE_R = crate :: BitReader < CTL2_REFMODE_A > ; # [doc = "This bit requests ULP_REF bandgap operation in static mode or sampled mode. The local reference buffer and 8-bit DAC inside comparator module are also configured accordingly. Static mode operation offers higher accuracy but consumes higher current. Sampled mode operation consumes lower current but with relaxed reference voltage accuracy. Comparator requests for reference voltage from ULP_REF only when REFLVL &amp;gt; 0.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum CTL2_REFMODE_A { # [doc = "0: STATIC"] CTL2_REFMODE_STATIC = 0 , # [doc = "1: SAMPLED"] CTL2_REFMODE_SAMPLED = 1 , } impl From < CTL2_REFMODE_A > for bool { # [inline (always)] fn from (variant : CTL2_REFMODE_A) -> Self { variant as u8 != 0 } } impl CTL2_REFMODE_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CTL2_REFMODE_A { match self . bits { false => CTL2_REFMODE_A :: CTL2_REFMODE_STATIC , true => CTL2_REFMODE_A :: CTL2_REFMODE_SAMPLED , } } # [doc = "STATIC"] # [inline (always)] pub fn is_ctl2_refmode_static (& self) -> bool { * self == CTL2_REFMODE_A :: CTL2_REFMODE_STATIC } # [doc = "SAMPLED"] # [inline (always)] pub fn is_ctl2_refmode_sampled (& self) -> bool { * self == CTL2_REFMODE_A :: CTL2_REFMODE_SAMPLED } } # [doc = "Field `CTL2_REFMODE` writer - This bit requests ULP_REF bandgap operation in static mode or sampled mode. The local reference buffer and 8-bit DAC inside comparator module are also configured accordingly. Static mode operation offers higher accuracy but consumes higher current. Sampled mode operation consumes lower current but with relaxed reference voltage accuracy. Comparator requests for reference voltage from ULP_REF only when REFLVL &amp;gt; 0."] pub type CTL2_REFMODE_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , CTL2_REFMODE_A > ; impl < 'a , REG , const O : u8 > CTL2_REFMODE_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "STATIC"] # [inline (always)] pub fn ctl2_refmode_static (self) -> & 'a mut crate :: W < REG > { self . variant (CTL2_REFMODE_A :: CTL2_REFMODE_STATIC) } # [doc = "SAMPLED"] # [inline (always)] pub fn ctl2_refmode_sampled (self) -> & 'a mut crate :: W < REG > { self . variant (CTL2_REFMODE_A :: CTL2_REFMODE_SAMPLED) } } # [doc = "Field `CTL2_REFSRC` reader - These bits select the reference source for the comparator."] pub type CTL2_REFSRC_R = crate :: FieldReader < CTL2_REFSRC_A > ; # [doc = "These bits select the reference source for the comparator.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum CTL2_REFSRC_A { # [doc = "0: OFF"] CTL2_REFSRC_OFF = 0 , # [doc = "1: VDDA_DAC"] CTL2_REFSRC_VDDA_DAC = 1 , # [doc = "2: VREF_DAC"] CTL2_REFSRC_VREF_DAC = 2 , # [doc = "3: VREF"] CTL2_REFSRC_VREF = 3 , } impl From < CTL2_REFSRC_A > for u8 { # [inline (always)] fn from (variant : CTL2_REFSRC_A) -> Self { variant as _ } } impl crate :: FieldSpec for CTL2_REFSRC_A { type Ux = u8 ; } impl CTL2_REFSRC_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CTL2_REFSRC_A { match self . bits { 0 => CTL2_REFSRC_A :: CTL2_REFSRC_OFF , 1 => CTL2_REFSRC_A :: CTL2_REFSRC_VDDA_DAC , 2 => CTL2_REFSRC_A :: CTL2_REFSRC_VREF_DAC , 3 => CTL2_REFSRC_A :: CTL2_REFSRC_VREF , _ => unreachable ! () , } } # [doc = "OFF"] # [inline (always)] pub fn is_ctl2_refsrc_off (& self) -> bool { * self == CTL2_REFSRC_A :: CTL2_REFSRC_OFF } # [doc = "VDDA_DAC"] # [inline (always)] pub fn is_ctl2_refsrc_vdda_dac (& self) -> bool { * self == CTL2_REFSRC_A :: CTL2_REFSRC_VDDA_DAC } # [doc = "VREF_DAC"] # [inline (always)] pub fn is_ctl2_refsrc_vref_dac (& self) -> bool { * self == CTL2_REFSRC_A :: CTL2_REFSRC_VREF_DAC } # [doc = "VREF"] # [inline (always)] pub fn is_ctl2_refsrc_vref (& self) -> bool { * self == CTL2_REFSRC_A :: CTL2_REFSRC_VREF } } # [doc = "Field `CTL2_REFSRC` writer - These bits select the reference source for the comparator."] pub type CTL2_REFSRC_W < 'a , REG , const O : u8 > = crate :: FieldWriterSafe < 'a , REG , 2 , O , CTL2_REFSRC_A > ; impl < 'a , REG , const O : u8 > CTL2_REFSRC_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "OFF"] # [inline (always)] pub fn ctl2_refsrc_off (self) -> & 'a mut crate :: W < REG > { self . variant (CTL2_REFSRC_A :: CTL2_REFSRC_OFF) } # [doc = "VDDA_DAC"] # [inline (always)] pub fn ctl2_refsrc_vdda_dac (self) -> & 'a mut crate :: W < REG > { self . variant (CTL2_REFSRC_A :: CTL2_REFSRC_VDDA_DAC) } # [doc = "VREF_DAC"] # [inline (always)] pub fn ctl2_refsrc_vref_dac (self) -> & 'a mut crate :: W < REG > { self . variant (CTL2_REFSRC_A :: CTL2_REFSRC_VREF_DAC) } # [doc = "VREF"] # [inline (always)] pub fn ctl2_refsrc_vref (self) -> & 'a mut crate :: W < REG > { self . variant (CTL2_REFSRC_A :: CTL2_REFSRC_VREF) } } # [doc = "Field `CTL2_REFSEL` reader - This bit selects if the selected reference voltage is applied to positive or negative terminal of the comparator."] pub type CTL2_REFSEL_R = crate :: BitReader < CTL2_REFSEL_A > ; # [doc = "This bit selects if the selected reference voltage is applied to positive or negative terminal of the comparator.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum CTL2_REFSEL_A { # [doc = "0: POSITIVE"] CTL2_REFSEL_POSITIVE = 0 , # [doc = "1: NEGATIVE"] CTL2_REFSEL_NEGATIVE = 1 , } impl From < CTL2_REFSEL_A > for bool { # [inline (always)] fn from (variant : CTL2_REFSEL_A) -> Self { variant as u8 != 0 } } impl CTL2_REFSEL_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CTL2_REFSEL_A { match self . bits { false => CTL2_REFSEL_A :: CTL2_REFSEL_POSITIVE , true => CTL2_REFSEL_A :: CTL2_REFSEL_NEGATIVE , } } # [doc = "POSITIVE"] # [inline (always)] pub fn is_ctl2_refsel_positive (& self) -> bool { * self == CTL2_REFSEL_A :: CTL2_REFSEL_POSITIVE } # [doc = "NEGATIVE"] # [inline (always)] pub fn is_ctl2_refsel_negative (& self) -> bool { * self == CTL2_REFSEL_A :: CTL2_REFSEL_NEGATIVE } } # [doc = "Field `CTL2_REFSEL` writer - This bit selects if the selected reference voltage is applied to positive or negative terminal of the comparator."] pub type CTL2_REFSEL_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , CTL2_REFSEL_A > ; impl < 'a , REG , const O : u8 > CTL2_REFSEL_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "POSITIVE"] # [inline (always)] pub fn ctl2_refsel_positive (self) -> & 'a mut crate :: W < REG > { self . variant (CTL2_REFSEL_A :: CTL2_REFSEL_POSITIVE) } # [doc = "NEGATIVE"] # [inline (always)] pub fn ctl2_refsel_negative (self) -> & 'a mut crate :: W < REG > { self . variant (CTL2_REFSEL_A :: CTL2_REFSEL_NEGATIVE) } } # [doc = "Field `CTL2_BLANKSRC` reader - These bits select the blanking source for the comparator."] pub type CTL2_BLANKSRC_R = crate :: FieldReader < CTL2_BLANKSRC_A > ; # [doc = "These bits select the blanking source for the comparator.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum CTL2_BLANKSRC_A { # [doc = "0: DISABLE"] CTL2_BLANKSRC_DISABLE = 0 , # [doc = "1: BLANKSRC1"] CTL2_BLANKSRC_BLANKSRC1 = 1 , # [doc = "2: BLANKSRC2"] CTL2_BLANKSRC_BLANKSRC2 = 2 , # [doc = "3: BLANKSRC3"] CTL2_BLANKSRC_BLANKSRC3 = 3 , # [doc = "4: BLANKSRC4"] CTL2_BLANKSRC_BLANKSRC4 = 4 , # [doc = "5: BLANKSRC5"] CTL2_BLANKSRC_BLANKSRC5 = 5 , # [doc = "6: BLANKSRC6"] CTL2_BLANKSRC_BLANKSRC6 = 6 , } impl From < CTL2_BLANKSRC_A > for u8 { # [inline (always)] fn from (variant : CTL2_BLANKSRC_A) -> Self { variant as _ } } impl crate :: FieldSpec for CTL2_BLANKSRC_A { type Ux = u8 ; } impl CTL2_BLANKSRC_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < CTL2_BLANKSRC_A > { match self . bits { 0 => Some (CTL2_BLANKSRC_A :: CTL2_BLANKSRC_DISABLE) , 1 => Some (CTL2_BLANKSRC_A :: CTL2_BLANKSRC_BLANKSRC1) , 2 => Some (CTL2_BLANKSRC_A :: CTL2_BLANKSRC_BLANKSRC2) , 3 => Some (CTL2_BLANKSRC_A :: CTL2_BLANKSRC_BLANKSRC3) , 4 => Some (CTL2_BLANKSRC_A :: CTL2_BLANKSRC_BLANKSRC4) , 5 => Some (CTL2_BLANKSRC_A :: CTL2_BLANKSRC_BLANKSRC5) , 6 => Some (CTL2_BLANKSRC_A :: CTL2_BLANKSRC_BLANKSRC6) , _ => None , } } # [doc = "DISABLE"] # [inline (always)] pub fn is_ctl2_blanksrc_disable (& self) -> bool { * self == CTL2_BLANKSRC_A :: CTL2_BLANKSRC_DISABLE } # [doc = "BLANKSRC1"] # [inline (always)] pub fn is_ctl2_blanksrc_blanksrc1 (& self) -> bool { * self == CTL2_BLANKSRC_A :: CTL2_BLANKSRC_BLANKSRC1 } # [doc = "BLANKSRC2"] # [inline (always)] pub fn is_ctl2_blanksrc_blanksrc2 (& self) -> bool { * self == CTL2_BLANKSRC_A :: CTL2_BLANKSRC_BLANKSRC2 } # [doc = "BLANKSRC3"] # [inline (always)] pub fn is_ctl2_blanksrc_blanksrc3 (& self) -> bool { * self == CTL2_BLANKSRC_A :: CTL2_BLANKSRC_BLANKSRC3 } # [doc = "BLANKSRC4"] # [inline (always)] pub fn is_ctl2_blanksrc_blanksrc4 (& self) -> bool { * self == CTL2_BLANKSRC_A :: CTL2_BLANKSRC_BLANKSRC4 } # [doc = "BLANKSRC5"] # [inline (always)] pub fn is_ctl2_blanksrc_blanksrc5 (& self) -> bool { * self == CTL2_BLANKSRC_A :: CTL2_BLANKSRC_BLANKSRC5 } # [doc = "BLANKSRC6"] # [inline (always)] pub fn is_ctl2_blanksrc_blanksrc6 (& self) -> bool { * self == CTL2_BLANKSRC_A :: CTL2_BLANKSRC_BLANKSRC6 } } # [doc = "Field `CTL2_BLANKSRC` writer - These bits select the blanking source for the comparator."] pub type CTL2_BLANKSRC_W < 'a , REG , const O : u8 > = crate :: FieldWriter < 'a , REG , 3 , O , CTL2_BLANKSRC_A > ; impl < 'a , REG , const O : u8 > CTL2_BLANKSRC_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "DISABLE"] # [inline (always)] pub fn ctl2_blanksrc_disable (self) -> & 'a mut crate :: W < REG > { self . variant (CTL2_BLANKSRC_A :: CTL2_BLANKSRC_DISABLE) } # [doc = "BLANKSRC1"] # [inline (always)] pub fn ctl2_blanksrc_blanksrc1 (self) -> & 'a mut crate :: W < REG > { self . variant (CTL2_BLANKSRC_A :: CTL2_BLANKSRC_BLANKSRC1) } # [doc = "BLANKSRC2"] # [inline (always)] pub fn ctl2_blanksrc_blanksrc2 (self) -> & 'a mut crate :: W < REG > { self . variant (CTL2_BLANKSRC_A :: CTL2_BLANKSRC_BLANKSRC2) } # [doc = "BLANKSRC3"] # [inline (always)] pub fn ctl2_blanksrc_blanksrc3 (self) -> & 'a mut crate :: W < REG > { self . variant (CTL2_BLANKSRC_A :: CTL2_BLANKSRC_BLANKSRC3) } # [doc = "BLANKSRC4"] # [inline (always)] pub fn ctl2_blanksrc_blanksrc4 (self) -> & 'a mut crate :: W < REG > { self . variant (CTL2_BLANKSRC_A :: CTL2_BLANKSRC_BLANKSRC4) } # [doc = "BLANKSRC5"] # [inline (always)] pub fn ctl2_blanksrc_blanksrc5 (self) -> & 'a mut crate :: W < REG > { self . variant (CTL2_BLANKSRC_A :: CTL2_BLANKSRC_BLANKSRC5) } # [doc = "BLANKSRC6"] # [inline (always)] pub fn ctl2_blanksrc_blanksrc6 (self) -> & 'a mut crate :: W < REG > { self . variant (CTL2_BLANKSRC_A :: CTL2_BLANKSRC_BLANKSRC6) } } # [doc = "Field `CTL2_DACCTL` reader - This bit determines if the comparator output or DACSW bit controls the selection between DACCODE0 and DACCODE1."] pub type CTL2_DACCTL_R = crate :: BitReader < CTL2_DACCTL_A > ; # [doc = "This bit determines if the comparator output or DACSW bit controls the selection between DACCODE0 and DACCODE1.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum CTL2_DACCTL_A { # [doc = "0: COMPOUT_SEL"] CTL2_DACCTL_COMPOUT_SEL = 0 , # [doc = "1: DACSW_SEL"] CTL2_DACCTL_DACSW_SEL = 1 , } impl From < CTL2_DACCTL_A > for bool { # [inline (always)] fn from (variant : CTL2_DACCTL_A) -> Self { variant as u8 != 0 } } impl CTL2_DACCTL_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CTL2_DACCTL_A { match self . bits { false => CTL2_DACCTL_A :: CTL2_DACCTL_COMPOUT_SEL , true => CTL2_DACCTL_A :: CTL2_DACCTL_DACSW_SEL , } } # [doc = "COMPOUT_SEL"] # [inline (always)] pub fn is_ctl2_dacctl_compout_sel (& self) -> bool { * self == CTL2_DACCTL_A :: CTL2_DACCTL_COMPOUT_SEL } # [doc = "DACSW_SEL"] # [inline (always)] pub fn is_ctl2_dacctl_dacsw_sel (& self) -> bool { * self == CTL2_DACCTL_A :: CTL2_DACCTL_DACSW_SEL } } # [doc = "Field `CTL2_DACCTL` writer - This bit determines if the comparator output or DACSW bit controls the selection between DACCODE0 and DACCODE1."] pub type CTL2_DACCTL_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , CTL2_DACCTL_A > ; impl < 'a , REG , const O : u8 > CTL2_DACCTL_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "COMPOUT_SEL"] # [inline (always)] pub fn ctl2_dacctl_compout_sel (self) -> & 'a mut crate :: W < REG > { self . variant (CTL2_DACCTL_A :: CTL2_DACCTL_COMPOUT_SEL) } # [doc = "DACSW_SEL"] # [inline (always)] pub fn ctl2_dacctl_dacsw_sel (self) -> & 'a mut crate :: W < REG > { self . variant (CTL2_DACCTL_A :: CTL2_DACCTL_DACSW_SEL) } } # [doc = "Field `CTL2_DACSW` reader - This bit selects between DACCODE0 and DACCODE1 to 8-bit DAC when DACCTL bit is 1."] pub type CTL2_DACSW_R = crate :: BitReader < CTL2_DACSW_A > ; # [doc = "This bit selects between DACCODE0 and DACCODE1 to 8-bit DAC when DACCTL bit is 1.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum CTL2_DACSW_A { # [doc = "0: DACCODE0_SEL"] CTL2_DACSW_DACCODE0_SEL = 0 , # [doc = "1: DACCODE1_SEL"] CTL2_DACSW_DACCODE1_SEL = 1 , } impl From < CTL2_DACSW_A > for bool { # [inline (always)] fn from (variant : CTL2_DACSW_A) -> Self { variant as u8 != 0 } } impl CTL2_DACSW_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CTL2_DACSW_A { match self . bits { false => CTL2_DACSW_A :: CTL2_DACSW_DACCODE0_SEL , true => CTL2_DACSW_A :: CTL2_DACSW_DACCODE1_SEL , } } # [doc = "DACCODE0_SEL"] # [inline (always)] pub fn is_ctl2_dacsw_daccode0_sel (& self) -> bool { * self == CTL2_DACSW_A :: CTL2_DACSW_DACCODE0_SEL } # [doc = "DACCODE1_SEL"] # [inline (always)] pub fn is_ctl2_dacsw_daccode1_sel (& self) -> bool { * self == CTL2_DACSW_A :: CTL2_DACSW_DACCODE1_SEL } } # [doc = "Field `CTL2_DACSW` writer - This bit selects between DACCODE0 and DACCODE1 to 8-bit DAC when DACCTL bit is 1."] pub type CTL2_DACSW_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , CTL2_DACSW_A > ; impl < 'a , REG , const O : u8 > CTL2_DACSW_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "DACCODE0_SEL"] # [inline (always)] pub fn ctl2_dacsw_daccode0_sel (self) -> & 'a mut crate :: W < REG > { self . variant (CTL2_DACSW_A :: CTL2_DACSW_DACCODE0_SEL) } # [doc = "DACCODE1_SEL"] # [inline (always)] pub fn ctl2_dacsw_daccode1_sel (self) -> & 'a mut crate :: W < REG > { self . variant (CTL2_DACSW_A :: CTL2_DACSW_DACCODE1_SEL) } } # [doc = "Field `CTL2_SAMPMODE` reader - Enable sampled mode of comparator."] pub type CTL2_SAMPMODE_R = crate :: BitReader < CTL2_SAMPMODE_A > ; # [doc = "Enable sampled mode of comparator.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum CTL2_SAMPMODE_A { # [doc = "0: DISABLE"] CTL2_SAMPMODE_DISABLE = 0 , # [doc = "1: ENABLE"] CTL2_SAMPMODE_ENABLE = 1 , } impl From < CTL2_SAMPMODE_A > for bool { # [inline (always)] fn from (variant : CTL2_SAMPMODE_A) -> Self { variant as u8 != 0 } } impl CTL2_SAMPMODE_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CTL2_SAMPMODE_A { match self . bits { false => CTL2_SAMPMODE_A :: CTL2_SAMPMODE_DISABLE , true => CTL2_SAMPMODE_A :: CTL2_SAMPMODE_ENABLE , } } # [doc = "DISABLE"] # [inline (always)] pub fn is_ctl2_sampmode_disable (& self) -> bool { * self == CTL2_SAMPMODE_A :: CTL2_SAMPMODE_DISABLE } # [doc = "ENABLE"] # [inline (always)] pub fn is_ctl2_sampmode_enable (& self) -> bool { * self == CTL2_SAMPMODE_A :: CTL2_SAMPMODE_ENABLE } } # [doc = "Field `CTL2_SAMPMODE` writer - Enable sampled mode of comparator."] pub type CTL2_SAMPMODE_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , CTL2_SAMPMODE_A > ; impl < 'a , REG , const O : u8 > CTL2_SAMPMODE_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "DISABLE"] # [inline (always)] pub fn ctl2_sampmode_disable (self) -> & 'a mut crate :: W < REG > { self . variant (CTL2_SAMPMODE_A :: CTL2_SAMPMODE_DISABLE) } # [doc = "ENABLE"] # [inline (always)] pub fn ctl2_sampmode_enable (self) -> & 'a mut crate :: W < REG > { self . variant (CTL2_SAMPMODE_A :: CTL2_SAMPMODE_ENABLE) } } impl R { # [doc = "Bit 0 - This bit requests ULP_REF bandgap operation in static mode or sampled mode. The local reference buffer and 8-bit DAC inside comparator module are also configured accordingly. Static mode operation offers higher accuracy but consumes higher current. Sampled mode operation consumes lower current but with relaxed reference voltage accuracy. Comparator requests for reference voltage from ULP_REF only when REFLVL &amp;gt; 0."] # [inline (always)] pub fn ctl2_refmode (& self) -> CTL2_REFMODE_R { CTL2_REFMODE_R :: new ((self . bits & 1) != 0) } # [doc = "Bits 3:4 - These bits select the reference source for the comparator."] # [inline (always)] pub fn ctl2_refsrc (& self) -> CTL2_REFSRC_R { CTL2_REFSRC_R :: new (((self . bits >> 3) & 3) as u8) } # [doc = "Bit 7 - This bit selects if the selected reference voltage is applied to positive or negative terminal of the comparator."] # [inline (always)] pub fn ctl2_refsel (& self) -> CTL2_REFSEL_R { CTL2_REFSEL_R :: new (((self . bits >> 7) & 1) != 0) } # [doc = "Bits 8:10 - These bits select the blanking source for the comparator."] # [inline (always)] pub fn ctl2_blanksrc (& self) -> CTL2_BLANKSRC_R { CTL2_BLANKSRC_R :: new (((self . bits >> 8) & 7) as u8) } # [doc = "Bit 16 - This bit determines if the comparator output or DACSW bit controls the selection between DACCODE0 and DACCODE1."] # [inline (always)] pub fn ctl2_dacctl (& self) -> CTL2_DACCTL_R { CTL2_DACCTL_R :: new (((self . bits >> 16) & 1) != 0) } # [doc = "Bit 17 - This bit selects between DACCODE0 and DACCODE1 to 8-bit DAC when DACCTL bit is 1."] # [inline (always)] pub fn ctl2_dacsw (& self) -> CTL2_DACSW_R { CTL2_DACSW_R :: new (((self . bits >> 17) & 1) != 0) } # [doc = "Bit 24 - Enable sampled mode of comparator."] # [inline (always)] pub fn ctl2_sampmode (& self) -> CTL2_SAMPMODE_R { CTL2_SAMPMODE_R :: new (((self . bits >> 24) & 1) != 0) } } impl W { # [doc = "Bit 0 - This bit requests ULP_REF bandgap operation in static mode or sampled mode. The local reference buffer and 8-bit DAC inside comparator module are also configured accordingly. Static mode operation offers higher accuracy but consumes higher current. Sampled mode operation consumes lower current but with relaxed reference voltage accuracy. Comparator requests for reference voltage from ULP_REF only when REFLVL &amp;gt; 0."] # [inline (always)] # [must_use] pub fn ctl2_refmode (& mut self) -> CTL2_REFMODE_W < CTL2_SPEC , 0 > { CTL2_REFMODE_W :: new (self) } # [doc = "Bits 3:4 - These bits select the reference source for the comparator."] # [inline (always)] # [must_use] pub fn ctl2_refsrc (& mut self) -> CTL2_REFSRC_W < CTL2_SPEC , 3 > { CTL2_REFSRC_W :: new (self) } # [doc = "Bit 7 - This bit selects if the selected reference voltage is applied to positive or negative terminal of the comparator."] # [inline (always)] # [must_use] pub fn ctl2_refsel (& mut self) -> CTL2_REFSEL_W < CTL2_SPEC , 7 > { CTL2_REFSEL_W :: new (self) } # [doc = "Bits 8:10 - These bits select the blanking source for the comparator."] # [inline (always)] # [must_use] pub fn ctl2_blanksrc (& mut self) -> CTL2_BLANKSRC_W < CTL2_SPEC , 8 > { CTL2_BLANKSRC_W :: new (self) } # [doc = "Bit 16 - This bit determines if the comparator output or DACSW bit controls the selection between DACCODE0 and DACCODE1."] # [inline (always)] # [must_use] pub fn ctl2_dacctl (& mut self) -> CTL2_DACCTL_W < CTL2_SPEC , 16 > { CTL2_DACCTL_W :: new (self) } # [doc = "Bit 17 - This bit selects between DACCODE0 and DACCODE1 to 8-bit DAC when DACCTL bit is 1."] # [inline (always)] # [must_use] pub fn ctl2_dacsw (& mut self) -> CTL2_DACSW_W < CTL2_SPEC , 17 > { CTL2_DACSW_W :: new (self) } # [doc = "Bit 24 - Enable sampled mode of comparator."] # [inline (always)] # [must_use] pub fn ctl2_sampmode (& mut self) -> CTL2_SAMPMODE_W < CTL2_SPEC , 24 > { CTL2_SAMPMODE_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct CTL2_SPEC ; impl crate :: RegisterSpec for CTL2_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`ctl2::R`](R) reader structure"] impl crate :: Readable for CTL2_SPEC { } # [doc = "`write(|w| ..)` method takes [`ctl2::W`](W) writer structure"] impl crate :: Writable for CTL2_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets CTL2 to value 0"] impl crate :: Resettable for CTL2_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }