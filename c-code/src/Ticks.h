/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#pragma once

#include <FreeRTOS.h>

static inline TickType_t TicksOfMSecs(uint32_t msecs)
{
  return msecs / portTICK_PERIOD_MS + !!(msecs % portTICK_PERIOD_MS);
}
