#ifndef _FSRS_H
#define _FSRS_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>


typedef enum fsrs_Rating {
  fsrs_Rating_Again = 1,
  fsrs_Rating_Hard = 2,
  fsrs_Rating_Good = 3,
  fsrs_Rating_Easy = 4,
} fsrs_Rating;

typedef enum fsrs_State {
  fsrs_State_New = 0,
  fsrs_State_Learning = 1,
  fsrs_State_Review = 2,
  fsrs_State_Relearning = 3,
} fsrs_State;

typedef struct fsrs_FSRS fsrs_FSRS;

typedef struct fsrs_ReviewLog {
  enum fsrs_Rating rating;
  int64_t elapsed_days;
  int64_t scheduled_days;
  enum fsrs_State state;
  int64_t reviewed_date_s;
} fsrs_ReviewLog;

typedef struct fsrs_Option_ReviewLog {
  struct fsrs_ReviewLog log;
  bool none;
} fsrs_Option_ReviewLog;

typedef struct fsrs_Card {
  const struct fsrs_Card *_0;
} fsrs_Card;

typedef struct fsrs_Fsrs {
  const struct fsrs_FSRS *_0;
} fsrs_Fsrs;

typedef struct fsrs_Parameters {
  float request_retention;
  int32_t maximum_interval;
  float w[19];
} fsrs_Parameters;

typedef struct fsrs_ScheduledCards {
  const struct fsrs_ScheduledCards *_0;
} fsrs_ScheduledCards;

struct fsrs_Option_ReviewLog fsrs_Card_log(const struct fsrs_Card *s);

struct fsrs_Card fsrs_Card_new(void);

struct fsrs_Fsrs fsrs_Fsrs_default(void);

struct fsrs_Fsrs fsrs_Fsrs_new(struct fsrs_Parameters p);

struct fsrs_ScheduledCards fsrs_Fsrs_schedule_timestamp(const struct fsrs_Fsrs *fsrs,
                                                        const struct fsrs_Card *card,
                                                        int64_t now);

struct fsrs_Card fsrs_ScheduledCards_select_card(const struct fsrs_ScheduledCards *self,
                                                 enum fsrs_Rating r);

#endif  /* _FSRS_H */
