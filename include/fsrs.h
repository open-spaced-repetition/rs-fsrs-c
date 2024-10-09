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

typedef struct fsrs_Card {
  const struct fsrs_Card *_0;
} fsrs_Card;

typedef struct fsrs_Fsrs {
  const struct fsrs_FSRS *_0;
} fsrs_Fsrs;

typedef struct fsrs_Parameters {
  double request_retention;
  int32_t maximum_interval;
  double w[19];
  double decay;
  double factor;
  bool enable_short_term;
} fsrs_Parameters;
#define fsrs_Parameters_DECAY -0.5
/**
 * (9/10) ^ (1 / DECAY) - 1
 */
#define fsrs_Parameters_FACTOR (19 / 81)

typedef struct fsrs_RecordLog {
  const struct fsrs_RecordLog *_0;
} fsrs_RecordLog;

typedef struct fsrs_SchedulingInfo {
  const struct fsrs_SchedulingInfo *_0;
} fsrs_SchedulingInfo;

typedef struct fsrs_ReviewLog {
  enum fsrs_Rating rating;
  int64_t elapsed_days;
  int64_t scheduled_days;
  enum fsrs_State state;
  int64_t reviewed_date_s;
} fsrs_ReviewLog;

double fsrs_Card_difficulty(const struct fsrs_Card *self);

int64_t fsrs_Card_due(const struct fsrs_Card *self);

int64_t fsrs_Card_elapsed_days(const struct fsrs_Card *self);

int32_t fsrs_Card_lapses(const struct fsrs_Card *self);

int64_t fsrs_Card_last_review(const struct fsrs_Card *self);

struct fsrs_Card fsrs_Card_new(void);

int32_t fsrs_Card_reps(const struct fsrs_Card *self);

int64_t fsrs_Card_scheduled_days(const struct fsrs_Card *self);

double fsrs_Card_stability(const struct fsrs_Card *self);

enum fsrs_State fsrs_Card_state(const struct fsrs_Card *self);

struct fsrs_Fsrs fsrs_Fsrs_default(void);

struct fsrs_Fsrs fsrs_Fsrs_new(struct fsrs_Parameters p);

struct fsrs_RecordLog fsrs_Fsrs_repeat_timestamp(const struct fsrs_Fsrs *fsrs,
                                                 const struct fsrs_Card *card,
                                                 int64_t now);

struct fsrs_SchedulingInfo fsrs_ScheduledCards_get(const struct fsrs_RecordLog *self,
                                                   enum fsrs_Rating r);

struct fsrs_Card fsrs_SchedulingInfo_card(const struct fsrs_SchedulingInfo *self);

struct fsrs_ReviewLog fsrs_SchedulingInfo_review_log(const struct fsrs_SchedulingInfo *self);

#endif  /* _FSRS_H */
