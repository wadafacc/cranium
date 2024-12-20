
/*
 *      ___           ___           ___           ___                       ___           ___
 *     /\  \         /\  \         /\  \         /\__\          ___        /\__\         /\__\
 *    /::\  \       /::\  \       /::\  \       /::|  |        /\  \      /:/  /        /::|  |
 *   /:/\:\  \     /:/\:\  \     /:/\:\  \     /:|:|  |        \:\  \    /:/  /        /:|:|  |
 *  /:/  \:\  \   /::\~\:\  \   /::\~\:\  \   /:/|:|  |__      /::\__\  /:/  /  ___   /:/|:|__|__
 * /:/__/ \:\__\ /:/\:\ \:\__\ /:/\:\ \:\__\ /:/ |:| /\__\  __/:/\/__/ /:/__/  /\__\ /:/ |::::\__\
 * \:\  \  \/__/ \/_|::\/:/  / \/__\:\/:/  / \/__|:|/:/  / /\/:/  /    \:\  \ /:/  / \/__/~~/:/  /
 *  \:\  \          |:|::/  /       \::/  /      |:/:/  /  \::/__/      \:\  /:/  /        /:/  /
 *   \:\  \         |:|\/__/        /:/  /       |::/  /    \:\__\       \:\/:/  /        /:/  /
 *    \:\__\        |:|  |         /:/  /        /:/  /      \/__/        \::/  /        /:/  /
 *     \/__/         \|__|         \/__/         \/__/                     \/__/         \/__/
 *
 *                                        C O M P I L E R
 *                                     ~ made by @wadafacc ~
 *
 */

#include <stdio.h>

main()
{

  char box[42069], *ptr = box, copy;

  *ptr += 8;
  while (*ptr)
  {
    ptr += 1;
    *ptr += 4;
    while (*ptr)
    {
      ptr += 1;
      *ptr += 2;
      ptr += 1;
      *ptr += 3;
      ptr += 1;
      *ptr += 3;
      ptr += 1;
      *ptr += 1;
      ptr += 4;
      *ptr -= 1;
    }
    ptr += 1;
    *ptr += 1;
    ptr += 1;
    *ptr += 1;
    ptr += 1;
    *ptr -= 1;
    ptr += 2;
    *ptr += 1;
    while (*ptr)
    {
      ptr += 1;
    }
    ptr += 1;
    *ptr -= 1;
  }
  ptr += 2;
  putchar(*ptr);
  ptr += 1;
  *ptr -= 3;
  putchar(*ptr);
  *ptr += 7;
  putchar(*ptr);
  putchar(*ptr);
  *ptr += 3;
  putchar(*ptr);
  ptr += 2;
  putchar(*ptr);
  ptr += 1;
  *ptr -= 1;
  putchar(*ptr);
  ptr += 1;
  putchar(*ptr);
  *ptr += 3;
  putchar(*ptr);
  *ptr -= 6;
  putchar(*ptr);
  *ptr -= 8;
  putchar(*ptr);
  ptr += 2;
  *ptr += 1;
  putchar(*ptr);
  ptr += 1;
  *ptr += 2;
  putchar(*ptr);

  exit();
}