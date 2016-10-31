/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#include "Task.h"

#define WORD_SIZE sizeof(uint32_t)

static inline size_t nwords_of_nbytes(size_t nbytes)
{
  return nbytes / WORD_SIZE + !!(nbytes % WORD_SIZE);
}

static inline uint16_t default_stack_size(void)
{
  return nwords_of_nbytes(2048);
}

uint16_t
TaskDefaultStackSize()
{
  return default_stack_size();
}
