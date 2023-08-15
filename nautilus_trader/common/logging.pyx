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

import platform
import socket
import sys
import traceback
from platform import python_version
from typing import Optional

import msgspec
import numpy as np
import pandas as pd
import psutil
import pyarrow
import pytz

from nautilus_trader import __version__

from libc.stdint cimport uint64_t

from nautilus_trader.common.clock cimport Clock
from nautilus_trader.common.logging cimport Logger
from nautilus_trader.core.correctness cimport Condition
from nautilus_trader.core.rust.common cimport LogColor
from nautilus_trader.core.rust.common cimport LogLevel
from nautilus_trader.core.rust.common cimport logger_get_trader_id_cstr
from nautilus_trader.core.rust.common cimport logger_log
from nautilus_trader.core.rust.common cimport logger_new
from nautilus_trader.core.string cimport cstr_to_pystr
from nautilus_trader.core.string cimport pybytes_to_cstr
from nautilus_trader.core.string cimport pystr_to_cstr
from nautilus_trader.core.uuid cimport UUID4
from nautilus_trader.model.identifiers cimport TraderId


RECV = "<--"
SENT = "-->"
CMD = "[CMD]"
EVT = "[EVT]"
DOC = "[DOC]"
RPT = "[RPT]"
REQ = "[REQ]"
RES = "[RES]"


cdef class Logger:
    """
    Provides a high-performance logger.

    Parameters
    ----------
    trader_id : TraderId, optional
        The trader ID for the logger.
    component : str, optional
        The component to which the logger belongs
    bypass : bool, default False
        If the log output is bypassed.
    """

    def __init__(
        self,
        TraderId trader_id = None,
        str component = None,
        bint bypass = False,
    ):
        if trader_id is None:
            trader_id = TraderId("TRADER-000")
        if component is None:
            component = "core"

        self._mem = logger_new(
            trader_id,
            pystr_to_cstr(component),
        )
        self.is_bypassed = bypass

    @property
    def trader_id(self) -> TraderId:
        """
        Return the loggers trader ID.

        Returns
        -------
        TraderId

        """
        return TraderId(cstr_to_pystr(logger_get_trader_id_cstr(&self._mem)))

    @property
    def component(self) -> str:
        """
        Return the loggers component name.

        Returns
        -------
        str

        """
        return self._component

    cdef void log(
        self,
        LogLevel level,
        str message,
    ):
        logger_log(
            &self._mem,
            level,
            pystr_to_cstr(message),
        )

    cpdef void debug(
        self,
        str message,
    ):
        """
        Log the given debug message with the logger.

        Parameters
        ----------
        message : str
            The log message content.
        color : LogColor, optional
            The log message color.
        annotations : dict[str, object], optional
            The annotations for the log record.

        """
        Condition.not_none(message, "message")

        if self.is_bypassed:
            return

        self.log(
            LogLevel.DEBUG,
            message,
        )

    cpdef void info(
        self, str message,
    ):
        """
        Log the given information message with the logger.

        Parameters
        ----------
        message : str
            The log message content.
        color : LogColor, optional
            The log message color.
        annotations : dict[str, object], optional
            The annotations for the log record.

        """
        Condition.not_none(message, "message")

        if self.is_bypassed:
            return

        self.log(
            LogLevel.INFO,
            message,
        )

    cpdef void warning(
        self,
        str message,
    ):
        """
        Log the given warning message with the logger.

        Parameters
        ----------
        message : str
            The log message content.
        color : LogColor, optional
            The log message color.
        annotations : dict[str, object], optional
            The annotations for the log record.

        """
        Condition.not_none(message, "message")

        if self.is_bypassed:
            return

        self.log(
            LogLevel.WARNING,
            message,
        )

    cpdef void error(
        self,
        str message,
    ):
        """
        Log the given error message with the logger.

        Parameters
        ----------
        message : str
            The log message content.
        color : LogColor, optional
            The log message color.
        annotations : dict[str, object], optional
            The annotations for the log record.

        """
        Condition.not_none(message, "message")

        if self.is_bypassed:
            return

        self.log(
            LogLevel.ERROR,
            message,
        )

    cpdef Logger with_component(self, str component):
        return Logger(self.trader_id, component)

cpdef void nautilus_header(Logger logger):
    Condition.not_none(logger, "logger")
    print("")  # New line to begin
    logger.info("\033[36m=================================================================")
    logger.info(f"\033[36m NAUTILUS TRADER - Automated Algorithmic Trading Platform")
    logger.info(f"\033[36m by Nautech Systems Pty Ltd.")
    logger.info(f"\033[36m Copyright (C) 2015-2023. All rights reserved.")
    logger.info("\033[36m=================================================================")
    logger.info("")
    logger.info("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣴⣶⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀")
    logger.info("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⣾⣿⣿⣿⠀⢸⣿⣿⣿⣿⣶⣶⣤⣀⠀⠀⠀⠀⠀")
    logger.info("⠀⠀⠀⠀⠀⠀⢀⣴⡇⢀⣾⣿⣿⣿⣿⣿⠀⣾⣿⣿⣿⣿⣿⣿⣿⠿⠓⠀⠀⠀⠀")
    logger.info("⠀⠀⠀⠀⠀⣰⣿⣿⡀⢸⣿⣿⣿⣿⣿⣿⠀⣿⣿⣿⣿⣿⣿⠟⠁⣠⣄⠀⠀⠀⠀")
    logger.info("⠀⠀⠀⠀⢠⣿⣿⣿⣇⠀⢿⣿⣿⣿⣿⣿⠀⢻⣿⣿⣿⡿⢃⣠⣾⣿⣿⣧⡀⠀⠀")
    logger.info("⠀⠀⠀⠀⢸⣿⣿⣿⣿⣆⠘⢿⣿⡿⠛⢉⠀⠀⠉⠙⠛⣠⣿⣿⣿⣿⣿⣿⣷⠀⠀")
    logger.info("⠀⠀⠀⠠⣾⣿⣿⣿⣿⣿⣧⠈⠋⢀⣴⣧⠀⣿⡏⢠⡀⢸⣿⣿⣿⣿⣿⣿⣿⡇⠀")
    logger.info("⠀⠀⠀⣀⠙⢿⣿⣿⣿⣿⣿⠇⢠⣿⣿⣿⡄⠹⠃⠼⠃⠈⠉⠛⠛⠛⠛⠛⠻⠇⠀")
    logger.info("⠀⠀⢸⡟⢠⣤⠉⠛⠿⢿⣿⠀⢸⣿⡿⠋⣠⣤⣄⠀⣾⣿⣿⣶⣶⣶⣦⡄⠀⠀⠀")
    logger.info("⠀⠀⠸⠀⣾⠏⣸⣷⠂⣠⣤⠀⠘⢁⣴⣾⣿⣿⣿⡆⠘⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀")
    logger.info("⠀⠀⠀⠀⠛⠀⣿⡟⠀⢻⣿⡄⠸⣿⣿⣿⣿⣿⣿⣿⡀⠘⣿⣿⣿⣿⠟⠀⠀⠀⠀")
    logger.info("⠀⠀⠀⠀⠀⠀⣿⠇⠀⠀⢻⡿⠀⠈⠻⣿⣿⣿⣿⣿⡇⠀⢹⣿⠿⠋⠀⠀⠀⠀⠀")
    logger.info("⠀⠀⠀⠀⠀⠀⠋⠀⠀⠀⡘⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠁⠀⠀⠀⠀⠀⠀⠀")
    logger.info("")
    logger.info("\033[36m=================================================================")
    logger.info("\033[36m SYSTEM SPECIFICATION")
    logger.info("\033[36m=================================================================")
    logger.info(f"CPU architecture: {platform.processor()}")
    try:
        cpu_freq_str = f"@ {int(psutil.cpu_freq()[2])} MHz"
    except Exception:  # noqa (historically problematic call on ARM)
        cpu_freq_str = None
    logger.info(f"CPU(s): {psutil.cpu_count()} {cpu_freq_str}")
    logger.info(f"OS: {platform.platform()}")
    log_memory(logger)
    logger.info("\033[36m=================================================================")
    logger.info("\033[36m IDENTIFIERS")
    logger.info("\033[36m=================================================================")
    logger.info(f"trader_id: {logger.trader_id}")
    logger.info(f"machine_id: {logger.machine_id}")
    logger.info(f"instance_id: {logger.instance_id}")
    logger.info("\033[36m=================================================================")
    logger.info("\033[36m VERSIONING")
    logger.info("\033[36m=================================================================")
    logger.info(f"nautilus-trader {__version__}")
    logger.info(f"python {python_version()}")
    logger.info(f"numpy {np.__version__}")
    logger.info(f"pandas {pd.__version__}")
    logger.info(f"msgspec {msgspec.__version__}")
    logger.info(f"psutil {psutil.__version__}")
    logger.info(f"pyarrow {pyarrow.__version__}")
    logger.info(f"pytz {pytz.__version__}")  # type: ignore
    try:
        import redis
        logger.info(f"redis {redis.__version__}")
    except ImportError:  # pragma: no cover
        redis = None
    try:
        import hiredis
        logger.info(f"hiredis {hiredis.__version__}")
    except ImportError:  # pragma: no cover
        hiredis = None
    try:
        import uvloop
        logger.info(f"uvloop {uvloop.__version__}")
    except ImportError:  # pragma: no cover
        uvloop = None

    logger.info("\033[36m=================================================================")

cpdef void log_memory(Logger logger):
    logger.info("\033[36m=================================================================")
    logger.info("\033[36m MEMORY USAGE")
    logger.info("\033[36m=================================================================")
    ram_total_mb = round(psutil.virtual_memory()[0] / 1000000)
    ram_used__mb = round(psutil.virtual_memory()[3] / 1000000)
    ram_avail_mb = round(psutil.virtual_memory()[1] / 1000000)
    ram_avail_pc = 100 - psutil.virtual_memory()[2]
    logger.info(f"RAM-Total: {ram_total_mb:,} MB")
    logger.info(f"RAM-Used:  {ram_used__mb:,} MB ({100 - ram_avail_pc:.2f}%)")
    if ram_avail_pc <= 50:
        logger.warning(f"RAM-Avail: {ram_avail_mb:,} MB ({ram_avail_pc:.2f}%)")
    else:
        logger.info(f"RAM-Avail: {ram_avail_mb:,} MB ({ram_avail_pc:.2f}%)")
