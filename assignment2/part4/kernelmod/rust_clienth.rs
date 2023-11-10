// #define _IOC_NRBITS	8
// #define _IOC_TYPEBITS	8
// #define _IOC_SIZEBITS	13
// #define _IOC_DIRBITS	3

const _IOC_NRBITS: u32 = 8;
const _IOC_TYPEBITS: u32 = 8;
const _IOC_SIZEBITS: u32 = 13;
const _IOC_DIRBITS: u32 = 3;

// #define _IOC(dir,type,nr,size)			\
// 	((unsigned int)				\
// 	 (((dir)  << _IOC_DIRSHIFT) |		\
// 	  ((type) << _IOC_TYPESHIFT) |		\
// 	  ((nr)   << _IOC_NRSHIFT) |		\
// 	  ((size) << _IOC_SIZESHIFT)))

// #[macro_export]
// macro_rules! _IOC {
//     ($dir:expr, $type:expr, $nr:expr, $size:expr) => {
//         (($dir << _IOC_DIRSHIFT)
//             | ($type << _IOC_TYPESHIFT)
//             | ($nr << _IOC_NRSHIFT)
//             | ($size << _IOC_SIZESHIFT))
//     };
// }
