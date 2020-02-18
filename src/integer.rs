//! Integer Instructions for PowerPC

/// Integer Instructions
pub enum IntegerInstruction {
    // add[x] - 31 
    Add(char),
    // addc[x] - 31
    AddC(char),
    // adde[x] - 31
    AddE(char),
    // addi - 14
    AddI(char),
    // addic - 12
    AddIc,
    // addic. - 13
    AddIcd,
    // addis - 15
    AddIs,
    // addme[x] - 31
    AddMe(char),
    // addze[x] - 31
    AddZe(char),
    // divw[x] - 31
    DivW(char),
    // divwu[x] - 31
    DivWu(char),
    // mulhw[x] - 31
    MulHw(char),
    // mulhwu[x] - 31
    MulHwu(char),
    // mulli - 07
    MulLi,
    // mullw[x] - 31
    MulLw(char),
    // neg[x] - 31
    Neg(char),
    // subf[x] - 31
    SubF(char),
    // subfc[x] - 31
    SubFc(char),
    // subfe[x] - 31
    SubFe(char),
    // subfic[x] - 08
    SubFic(char),
    // subfme[x] - 31
    SubFme(char),
    // subfze[x] - 31
    SubFze(char),
}
