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

typedef struct fsrs_ReviewLog {
  enum fsrs_Rating rating;
  int64_t elapsed_days;
  int64_t scheduled_days;
  enum fsrs_State state;
  int64_t reviewed_date_s;
} fsrs_ReviewLog;

typedef struct fsrs_card {
  const struct Card *inner;
} fsrs_card;

typedef struct fsrs_fsrs {
  const struct FSRS *inner;
} fsrs_fsrs;

typedef struct fsrs_ScheduledCards {
  const struct ScheduledCards *inner;
} fsrs_ScheduledCards;

struct fsrs_ReviewLog fsrs_ReviewLog_log(const struct fsrs_card *s);

struct fsrs_card fsrs_card_new(void);

struct fsrs_fsrs fsrs_fsrs_new(void);

struct fsrs_ScheduledCards fsrs_schedule_s(const struct fsrs_fsrs *fsrs,
                                           const struct fsrs_card *card,
                                           int64_t now);

struct fsrs_card select_card(const struct fsrs_ScheduledCards *self, enum fsrs_Rating r);

#endif  /* _FSRS_H */
