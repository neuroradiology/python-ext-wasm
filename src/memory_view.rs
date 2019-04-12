//! The `Buffer` Python object to build WebAssembly values.

use std::mem::size_of;
use pyo3::{
    prelude::*,
    class::PySequenceProtocol,
    exceptions::IndexError,
};
use wasmer_runtime::memory::Memory;

macro_rules! memory_view {
    ($class_name:ident over $wasm_type:ty | $bytes_per_element:expr) => {
        #[pyclass]
        pub struct $class_name {
            pub memory: Memory,
            pub offset: usize,
        }

        #[pymethods]
        impl $class_name {
            #[getter]
            fn bytes_per_element(&self) -> PyResult<u8> {
                Ok($bytes_per_element)
            }
        }

        #[pyproto]
        impl PySequenceProtocol for $class_name {
            fn __len__(&self) -> PyResult<usize> {
                Ok(self.memory.view::<$wasm_type>()[self.offset..].len() / size_of::<$wasm_type>())
            }

            fn __getitem__(&self, index: isize) -> PyResult<$wasm_type> {
                let offset = self.offset;
                let view = self.memory.view::<$wasm_type>();

                if index < 0 {
                    return Err(IndexError::py_err("Out of bound: Index cannot be negative."))
                }

                let index = index as usize;

                if view.len() <= offset + index {
                    Err(
                        IndexError::py_err(
                            format!(
                                "Out of bound: Absolute index {} is larger than the memory size {}.",
                                offset + index,
                                view.len()
                            )
                        )
                    )
                } else {
                    Ok(view[offset + index].get())
                }
            }

            /*
            fn __setitem__(&mut self, index: isize, value: u8) -> PyResult<()> {
                let offset = self.offset;
                let view = self.memory.view::<u8>();

                if index < 0 {
                    return Err(IndexError::py_err("Out of bound: Index cannot be negative."))
                }

                let index = index as usize;

                if view.len() <= offset + index {
                    Err(
                        IndexError::py_err(
                            format!(
                                "Out of bound: Absolute index {} is larger than the memory size {}.",
                                offset + index,
                                view.len()
                            )
                        )
                    )
                } else {
                    view[offset + index].set(value);

                    Ok(())
                }
            }
            */
        }
    }
}

memory_view!(Uint8MemoryView over u8|1);
memory_view!(Int8MemoryView over i8|1);
memory_view!(Uint16MemoryView over u16|2);
memory_view!(Int16MemoryView over i16|2);
memory_view!(Uint32MemoryView over u32|4);
memory_view!(Int32MemoryView over i32|4);
