#include "../include/fsrs.h"
#include <stdint.h>
#include <stdio.h>
#include <time.h>

int main(void) {
  fsrs_Fsrs F1 = fsrs_Fsrs_new(
      (fsrs_Parameters){.maximum_interval = 36000,
                        .request_retention = 0.9,
                        .w = {0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                              0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0},
                        .decay = 0.0,
                        .factor = 0.0,
                        .enable_short_term = false});
  fsrs_Card c = fsrs_Card_new();
  fsrs_Fsrs F = fsrs_Fsrs_default();
  time_t t = time(NULL);

  fsrs_RecordLog scheduled_cards = fsrs_Fsrs_repeat_timestamp(&F, &c, t);
  const fsrs_Rating ratings[] = {fsrs_Rating_Again, fsrs_Rating_Hard,
                                 fsrs_Rating_Good, fsrs_Rating_Easy};
  for (int i = 0; i < 4; i++) {
    fsrs_Rating rating = ratings[i];
    fsrs_SchedulingInfo scheduling_info =
        fsrs_ScheduledCards_get(&scheduled_cards, rating);
    fsrs_Card card = fsrs_SchedulingInfo_card(&scheduling_info);
    fsrs_ReviewLog log = fsrs_SchedulingInfo_review_log(&scheduling_info);

    printf("scheduled_days: %ld\nelapsed_days: %ld\ndate: %srating: %d\nstate: "
           "%d\n",
           log.scheduled_days, log.elapsed_days,
           asctime(localtime(&log.reviewed_date_s)), log.rating, log.state);
    printf("card:\n  elapsed_days: %ld\n  scheduled_days: %ld\n  due: %ld\n  "
           "stability: %f\n  difficulty: %f\n  reps: %d\n  lapses: %d\n  "
           "state: %u\n  last_review: %ld\n",
           fsrs_Card_elapsed_days(&card), fsrs_Card_scheduled_days(&card),
           fsrs_Card_due(&card), fsrs_Card_stability(&card),
           fsrs_Card_difficulty(&card), fsrs_Card_reps(&card),
           fsrs_Card_lapses(&card), fsrs_Card_state(&card),
           fsrs_Card_last_review(&card));
    return 0;
  }
}
