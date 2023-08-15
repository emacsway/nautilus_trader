# -------------------------------------------------------------------------------------------------
#  Copyright (C) 2015-2023 Nautech Systems Pty Ltd. All rights reserved.
#  https://nautechsystems.io
#
#  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
#  You may not use this file except in compliance with the License.
#  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
#
#  Unless required by applicable law or agreed to in writing, software
#  distributed under the License is distributed on an "AS IS" BASIS,
#  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
#  See the License for the specific language governing permissions and
#  limitations under the License.
# -------------------------------------------------------------------------------------------------

from typing import Callable

from libc.stdint cimport uint64_t

from nautilus_trader.common.clock cimport Clock
from nautilus_trader.common.logging cimport Logger
from nautilus_trader.core.rust.common cimport LogColor
from nautilus_trader.core.rust.common cimport Logger_t
from nautilus_trader.core.rust.common cimport LogLevel


cdef str RECV
cdef str SENT
cdef str CMD
cdef str EVT
cdef str DOC
cdef str RPT
cdef str REQ
cdef str RES


cdef class Logger:
    cdef Logger_t _mem
    cdef bint is_bypassed

    cdef void log(
        self,
        LogLevel level,
        str message,
    )

    cpdef void debug(self, str message)
    cpdef void info(self, str message)
    cpdef void warning(self, str message)
    cpdef void error(self, str message)
    cpdef Logger with_component(self, str component)


cpdef void nautilus_header(Logger logger)
cpdef void log_memory(Logger logger)
