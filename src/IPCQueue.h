/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#pragma once

/*
 * Inter-Process Communication is one of the building blocks of the
 * firmware.
 *
 * IPC is performed between tasks (not processes) by exchanging IPC
 * messages. A task can either send messages, receive messages, or
 * both.
 *
 * IPC messages
 * ------------
 *
 * An IPC message is represented by the data structure IPCMessage. Each
 * message can transfer two 32-bit values and optionally an external
 * buffer.
 *
 * Initialize the message structure by calling IPCMessageInit(). To
 * release an initialized message's internal resource, call
 * IPCMessageUninit()
 *
 *    IPCMessage msg;
 *    IPCMessageInit(&msg);
 *    // do IPC
 *    IPCMessageUninit(&msg);
 *
 * The init function returns a negative value on errors, or 0 on
 * success. In the examples, we leave out error checking, but don't do
 * so in production code.
 *
 * After you initialized the message, you have to 'produce' it, give
 * it to a consumer, and wait for consumption. The produce step is
 * performed by IPCMessageProduce(), the waiting step is performed by
 * IPCMessageWaitForConsumption().
 *
 *    const char buf[] = "Hello world";
 *    IPCMessage msg;
 *    IPCMessageInit(&msg);
 *    IPCMessageProduce(&msg, sizeof(buf), (void*)buf);
 *    // do message setup and actual IPC with the consumer
 *    IPCMessageWaitForConsumption(&msg)
 *    IPCMessageUninit(&msg);
 *
 * IPCMessageProduce() takes the message as its argument, and an optional
 * buffer plus length. The buffer's address is attached to the message and
 * received by the consumer. Pass 0 and NULL if you don't want to transfer
 * a buffer. IPC messages do not transfer ownership of the message or the
 * attached buffer! The message, the buffer and the buffer's content must
 * be valid until the consumer has finished processing the message.
 *
 * Two additional values can be transfered in the IPC message.
 *
 *    msg.mDWord0 = (uint32_t)1ul;
 *    msg.mDWord1 = (uint32_t)-1l;
 *
 * After the IPC message has been given to the consumer, which is described
 * in the next section, IPCMessageWaitForConsumption() allows to wait for the
 * completion of the consumer's side. Instead of only consuming a message,
 * producers have the option of returning a reply. Replace the call to
 * IPCMessageWaitForConsumption() with IPCMessageWaitForReply() to receive
 * the reply. The reply data can contain two 32-bit values and optionally a
 * buffer. Again, ownership of the buffer is not transfered.
 *
 * An initialized IPC message can be used throughout multiple produce-
 * consume cyles. There's no requirement to uninitialize and re-initialize
 * after consumption.
 *
 * IPC is performed asynchronously. Both, producer and consumer, continue
 * independently. Only calling IPCMessageWaitForConsumption() or
 * IPCMessageWaitForReply() will synchronize them. Once these calls return,
 * it's safe to release the message, buffer, and buffer content.
 *
 * There's one shortcut through the produce-consume cycle. If you only want
 * to send a message to a consumer and don't have to care about a reply or
 * buffer lifetime, you can set NOWAIT on the produced message.
 *
 *    msg.mStatus |= IPC_MESSAGE_FLAG_NOWAIT
 *
 * NOWAIT is an optimization for these single-shot use cases. The producer
 * will not reply after consuming the message, and producers will not wait
 * for it. Calling the related functions is safe, but there's no reqirement
 * to do so.
 *
 * Sending a message
 * -----------------
 *
 * Each consumer task waits for messages on a queue of type IPCMessageQueue.
 * To insert an IPC message into the queue, call IPCMessageQueueConsume(). This
 * will wake up the waiting consumer.
 *
 *    extern IPCMessageQueue msgQueue;
 *
 *    const char buf[] = "Hello world";
 *    IPCMessage msg;
 *    IPCMessageInit(&msg);
 *    IPCMessageProduce(&msg, sizeof(buf), (void*)buf);
 *    // do message setup
 *    IPCMessageQueueConsume(&msgQueue, &msg);
 *    IPCMessageWaitForConsumption(&msg)
 *    IPCMessageUninit(&msg);
 *
 * Calls to IPCMessageQuueConsume() are meant to complete quickly. But
 * if there's lots of contention on the queue, or the consumer is slow,
 * the function might block until there's space available at the end of
 * the message queue.
 *
 * Receiving a message
 * -------------------
 *
 * The consumer task waits for incomming messages on an IPCMessageQueue
 * until a message arrives. Message queues are initialized with a call to
 * IPCMessageQueueInit().
 *
 *    IPCMessageQueue msgQueue;
 *    IPCMessageQueueInit(&msgQueue);
 *
 * Waiting if performed by IPCMessageQueueWait(). The function's message
 * argument returns the received message. After processing the message, call
 * IPCMessageConsume() to signal the producer that you're done.
 *
 *    IPCMessageQueue msgQueue;
 *    IPCMessageQueueInit(&msgQueue);
 *
 *    while (1) {
 *      IPCMessage msg;
 *      IPCMessageQueueWait(&msgQueue, &msg);
 *      // process message
 *      IPCMessageConsume(&msg);
 *    }
 *
 * To send a reply to the producer, replace the call to IPCMessageConsume()
 * with IPCMessageReply(). This function allows to transfer two 32-bit values,
 * and a buffer to the producer. Again, buffer ownership is not transfered.
 */

#include <FreeRTOS.h>
#include <queue.h>

#include <stdint.h>

/*
 * IPCMessage
 */

enum IPCMessageStatus {
  /* Don't wait for consumer and don't signal consumption to producer. */
  IPC_MESSAGE_FLAG_NOWAIT     = 0x01000000,
  IPC_MESSAGE_STATE_CLEAR    = 0x00000000,
  IPC_MESSAGE_STATE_PRODUCED = 0x10000000,
  IPC_MESSAGE_STATE_PENDING  = 0x20000000,
  IPC_MESSAGE_STATE_ERROR    = 0x30000000
};

typedef struct
{
  /* Internal monitor for signalling */
  QueueHandle_t mMonitor;

  /* Two dword for data transfers. */
  uint32_t mDWord0;
  uint32_t mDWord1;
  /* Message state [31:28], flags [27:24], and buffer length [23:0] */
  uint32_t mStatus;
  /* Message buffer */
  void* mBuffer;
} IPCMessage;

int
IPCMessageInit(IPCMessage* aMsg);

void
IPCMessageUninit(IPCMessage* aMsg);

uint32_t
IPCMessageGetBufferLength(const IPCMessage* aMsg);

int
IPCMessageProduce(IPCMessage* aMsg, uint32_t aLength, void* aBuffer);

int
IPCMessageWaitForReply(IPCMessage* aMsg);

int
IPCMessageWaitForConsumption(IPCMessage* aMsg);

int
IPCMessageConsumeAndReply(IPCMessage* aMsg,
                          uint32_t aDWord0, uint32_t aDWord1,
                          uint32_t aFlags, uint32_t aLength,
                          void* aBuffer);

int
IPCMessageConsume(IPCMessage* aMsg);

/*
 * IPCMessageQueue
 */

typedef struct
{
  QueueHandle_t mWaitQueue;
} IPCMessageQueue;

int
IPCMessageQueueInit(IPCMessageQueue* aMsgQueue);

int
IPCMessageQueueConsume(IPCMessageQueue* aMsgQueue,
                       IPCMessage* aMsg);

int
IPCMessageQueueWait(IPCMessageQueue* aMsgQueue,
                    IPCMessage* aMsg);
