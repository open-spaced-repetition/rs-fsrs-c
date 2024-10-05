#ifndef _FSRS_H
#define _FSRS_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>


typedef enum fsrs_Rating {
  Again = 1,
  Hard = 2,
  Good = 3,
  Easy = 4,
} fsrs_Rating;

typedef enum fsrs_State {
  New = 0,
  Learning = 1,
  Review = 2,
  Relearning = 3,
} fsrs_State;

typedef struct Card Card;

typedef struct FSRS FSRS;

typedef struct ScheduledCards ScheduledCards;

typedef struct fsrs_Card {
  const struct Card *_0;
} fsrs_Card;

typedef struct fsrs_Fsrs {
  const struct FSRS *_0;
} fsrs_Fsrs;

typedef struct fsrs_Parameters {
  float request_retention;
  int32_t maximum_interval;
  float w[19];
} fsrs_Parameters;

typedef struct fsrs_ReviewLog {
  enum fsrs_Rating rating;
  int64_t elapsed_days;
  int64_t scheduled_days;
  enum fsrs_State state;
  int64_t reviewed_date_s;
} fsrs_ReviewLog;

typedef struct fsrs_ScheduledCards {
  const struct ScheduledCards *_0;
} fsrs_ScheduledCards;

struct fsrs_Card fsrs_Card_new(void);

struct fsrs_Fsrs fsrs_Fsrs_default(void);

struct fsrs_Fsrs fsrs_Fsrs_new(struct fsrs_Parameters p);

struct fsrs_ReviewLog fsrs_get_ReviewLog(const struct fsrs_Card *s);

struct fsrs_ScheduledCards fsrs_schedule_timestamp(const struct fsrs_Fsrs *fsrs,
                                                   const struct fsrs_Card *card,
                                                   int64_t now);

struct fsrs_Card select_card(const struct fsrs_ScheduledCards *self, enum fsrs_Rating r);

#endif  /* _FSRS_H */
