#include "../include/fsrs.h"
#include <stdint.h>
#include <stdio.h>
#include <time.h>

int main(void) {
  fsrs_Fsrs F1 = fsrs_Fsrs_new(
      (fsrs_Parameters){.maximum_interval = 36000,
                        .request_retention = 0.9,
                        .w = {0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                              0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0}});
  fsrs_Card c = fsrs_Card_new();
  fsrs_Fsrs F = fsrs_Fsrs_default();
  time_t t = time(NULL);

  fsrs_ScheduledCards scheduled_cards = fsrs_Fsrs_schedule_timestamp(&F, &c, t);
  fsrs_Rating r = fsrs_Rating_Easy;
  fsrs_Card updated_card = fsrs_ScheduledCards_select_card(&scheduled_cards, r);
  fsrs_Option_ReviewLog option_log = fsrs_Card_log(&updated_card);
  if (option_log.none) {
    printf("No log\n");
    return 0;
  }
  fsrs_ReviewLog log = option_log.log;
  printf("scheduled_days: %ld, elapsed_days: %ld, timestamp: %ld, rating: "
         "%d, state: %d\n",
         log.scheduled_days, log.elapsed_days, log.reviewed_date_s, log.rating,
         log.state);
  puts(asctime(localtime(&log.reviewed_date_s)));
  return 0;
}