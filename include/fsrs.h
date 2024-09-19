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

typedef struct Card Card;

typedef struct FSRS FSRS;

typedef struct ScheduledCards ScheduledCards;

typedef struct fsrs_card {
  const struct Card *inner;
} fsrs_card;

typedef struct fsrs_fsrs {
  const struct FSRS *inner;
} fsrs_fsrs;

typedef struct fsrs_ScheduledCards {
  const struct ScheduledCards *inner;
} fsrs_ScheduledCards;

struct fsrs_card fsrs_card_new(void);

struct fsrs_fsrs fsrs_fsrs_new(void);

struct fsrs_ScheduledCards fsrs_schedule(const struct fsrs_fsrs *fsrs,
                                         const struct fsrs_card *card);

int64_t fsrs_schedule_debug(const struct fsrs_card *self);

struct fsrs_card select_card(const struct fsrs_ScheduledCards *self, enum fsrs_Rating r);

#endif  /* _FSRS_H */
