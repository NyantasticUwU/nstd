#ifndef NSTD_NSTD_H_INCLUDED
#define NSTD_NSTD_H_INCLUDED
#if defined(__WINDOWS__)\
    || defined(_WIN32)\
    || defined(_WIN64)\
    || defined(__WIN32__)\
    || defined(__TOS_WIN__)
#define NSTDAPI __declspec(dllexport)
#else
#define NSTDAPI
#endif
#ifndef __cplusplus
#if __STDC__ != 1
#error "The compiler you are using is not C standard compliant."
#endif
#else
#define NSTDCPP
#endif
#endif
