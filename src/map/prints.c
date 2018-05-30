/*
** RUDREAIS, 05/27/18
** roguelike
** File description:
** prints function for map
*/

#include <ncurses.h>
#include "board.h"

/*
void print_window(tab_t *tab)
{
	box(tab->win, 0, 0);
	wrefresh(tab->win);
}

void print_map_window(tab_t *tab, floor_t *f_floor, cursor_t *map_pos, cursor_t *pos)
{
	for (int i = 0; i < f_floor->ymax - 1; i++) {
		wmove(tab->win, map_pos->y + i, map_pos->x);
		wprintw(tab->win, "%s", f_floor->design[i]);
	}
	mvwprintw(tab->win, (map_pos->y - 1) + pos->y, (map_pos->x - 1) + pos->x, "@");
}

void debug_map(char **map, int ymax)
{
	for (int i = 0; i < ymax; i++)
		printf("%s", map[i]);
}
*/

void print_window(tab_t *tab)
{

}

void print_map_window(tab_t *tab, floor_t *f_floor, cursor_t *map_pos)
{

}

void print_game(tab_t *tab, floor_t *f_floor, cursor_t *map_pos, cursor_t *pos)
{
		wclear(tab->win);
		print_map_window(tab, f_floor, map_pos);
		print_window(tab);
}

void print_tabs(tab_t **tabs, int nb_tab)
{
	for (int i = 0; i < nb_tab; i++) {
		box(tabs[i]->win, 0, 0);
		wrefresh(tabs[i]->win);
	}
}