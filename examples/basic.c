#include "../include/fsrs.h"
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <time.h>

int main(void) {
  fsrs_Fsrs F = fsrs_Fsrs_new(
      (fsrs_Parameters){.maximum_interval = 36000,
                        .request_retention = 0.9,
                        .w = {0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                              0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0},
                        .decay = 0.0,
                        .factor = 0.0,
                        .enable_short_term = false});
  const fsrs_Card c = fsrs_Card_new();
  F = fsrs_Fsrs_default();
  const time_t t = time(NULL);

  const fsrs_RecordLog scheduled_cards = fsrs_Fsrs_repeat_timestamp(&F, &c, t);
  const fsrs_Rating ratings[] = {fsrs_Rating_Again, fsrs_Rating_Hard,
                                 fsrs_Rating_Good, fsrs_Rating_Easy};
  for (int i = 0; i < 4; i++) {
    const fsrs_Rating rating = ratings[i];
    const fsrs_SchedulingInfo scheduling_info =
        fsrs_ScheduledCards_get(&scheduled_cards, rating);
    const fsrs_Card card = fsrs_SchedulingInfo_card(&scheduling_info);
    const fsrs_ReviewLog review_log =
        fsrs_SchedulingInfo_review_log(&scheduling_info);
    const time_t reviewed_date = fsrs_ReviewLog_reviewed_date(&review_log);
    printf("review_log:\n"
           "  scheduled_days: %" PRId64 "\n"
           "  elapsed_days: %" PRId64 "\n"
           "  date: %s"
           "  rating: %" PRId32 "\n"
           "  state: %" PRId32 "\n",
           fsrs_ReviewLog_scheduled_days(&review_log),
           fsrs_ReviewLog_elapsed_days(&review_log),
           asctime(localtime(&reviewed_date)),
           fsrs_ReviewLog_rating(&review_log),
           fsrs_ReviewLog_state(&review_log));
    printf("card:\n"
           "  elapsed_days: %" PRId64 "\n"
           "  scheduled_days: %" PRId64 "\n"
           "  due: %" PRId64 "\n"
           "  stability: %f\n"
           "  difficulty: %f\n"
           "  reps: %" PRId32 "\n"
           "  lapses: %" PRId32 "\n"
           "  state: %" PRIu32 "\n"
           "  last_review: %" PRId64 "\n",
           fsrs_Card_elapsed_days(&card), fsrs_Card_scheduled_days(&card),
           fsrs_Card_due(&card), fsrs_Card_stability(&card),
           fsrs_Card_difficulty(&card), fsrs_Card_reps(&card),
           fsrs_Card_lapses(&card), fsrs_Card_state(&card),
           fsrs_Card_last_review(&card));
  }
  return 0;
}
