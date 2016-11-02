/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#include <hw_ints.h>
#include <hw_types.h>
#include <interrupt.h>
#include <prcm.h>
#include <rom_map.h>
#include <stdlib.h>

#include "pinmux.h"
#include "Producer.h"
#include "Serial.h"

extern void (* const g_pfnVectors[])(void);

int
main(void)
{
  /*
   * Initialize board
   */

  // Set vector table base
  MAP_IntVTableBaseSet((unsigned long)g_pfnVectors);

  // Enable Processor
  MAP_IntMasterEnable();
  MAP_IntEnable(FAULT_SYSTICK);

  PRCMCC3200MCUInit();

  PinMuxConfig();

  /*
   * Create the output task
   */

  static SerialOutTask serialOutTask;

  if (SerialOutTaskInit(&serialOutTask) < 0) {
    return EXIT_FAILURE;
  }
  if (SerialOutTaskSpawn(&serialOutTask) < 0) {
    return EXIT_FAILURE;
  }

  /*
   * Create the producer task
   */

  static ProducerTask producerTask;

  if (ProducerTaskInit(&producerTask, &serialOutTask.mRecvQueue) < 0) {
    return EXIT_FAILURE;
  }
  if (ProducerTaskSpawn(&producerTask) < 0) {
    return EXIT_FAILURE;
  }

  /*
   * Start the task scheduler
   */

  vTaskStartScheduler();

  return EXIT_SUCCESS;
}
