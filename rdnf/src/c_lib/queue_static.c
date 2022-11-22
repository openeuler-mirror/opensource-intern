#include <solv/queue.h>
#include <solv/dataiterator.h>
#include <solv/pool.h>
#include <solv/repo.h>
char* char_ptr_offset_bind(char* p,int offset){

  return p+offset;
}
void queue_empty_static(Queue *q)
{
  if (q->alloc)
  {
    q->left += (q->elements - q->alloc) + q->count;
    q->elements = q->alloc;
  }
  else
    q->left += q->count;
  q->count = 0;
}

void queue_push_static(Queue *q, Id id)
{
  if (!q->left)
    queue_alloc_one(q);
  q->elements[q->count++] = id;
  q->left--;
}

void queue_push2_static(Queue *q, Id id1, Id id2)
{
  queue_push(q, id1);
  queue_push(q, id2);
}

Dataiterator create_data_iterator_empty_bind()
{
  Dataiterator di = {0};
  return di;
}

Repo create_Repo_empty_bind()
{
  Repo r = {0};
  return r;
}

void dataiterator_init_simple_bind(Dataiterator *di, Pool *pool, const char *match, int flags)
{
  dataiterator_init(di, pool, 0, 0, 0, match, flags);
}

void dataiterator_set_search_simple_bind(Dataiterator *di)
{
  dataiterator_set_search(di, 0, 0);
}

int get_queue_element_value_bind(Queue *q, int index)
{
  if (index >= q->count)
  {
    return -1;
  };
  return q->elements[index];
}

Solvable *pool_id2solvable_static(const Pool *pool, Id p)
{
  return pool->solvables + p;
}
void solv_add_flags_to_jobs_bind(Queue *q, int flags)
{
  for (int i = 0; i < q->count; i += 2)
  {
    q->elements[i] |= flags;
  }
}
Solvable *get_pool_solvables_value_bind(const Pool *pool, unsigned int index)
{
  return &(pool->solvables[index]);
}
int is_pseudo_package_static(Pool *pool, Solvable *s)
{
  const char *n = pool_id2str(pool, s->name);
  if (*n == 'p' && !strncmp(n, "patch:", 6))
  {
    return 1;
  }
  return 0;
}
Repo *pool_id2repo_static(Pool *pool, Id repoid)
{
  return repoid < pool->nrepos ? pool->repos[repoid] : NULL;
}
Id pool_whatprovides_static(Pool *pool, Id d)
{
  if (!ISRELDEP(d))
  {
    if (pool->whatprovides[d])
      return pool->whatprovides[d];
  }
  else
  {
    Id v = GETRELID(d);
    if (pool->whatprovides_rel[v])
      return pool->whatprovides_rel[v];
  }
  return pool_addrelproviders(pool, d);
}
Id get_pool_whatprovidesdata_value_bind(Pool *pool, Id index)
{
  return pool->whatprovidesdata[index];
}
int pool_match_nevr_static(Pool *pool, Solvable *s, Id d)
{
  if (!ISRELDEP(d))
    return d == s->name;
  else
    return pool_match_nevr_rel(pool, s, d);
}
int pool_disabled_solvable_static(const Pool *pool, Solvable *s)
{
  if (s->repo && s->repo->disabled)
    return 1;
  if (pool->considered)
  {
    Id id = s - pool->solvables;
    if (!MAPTST(pool->considered, id))
      return 1;
  }
  return 0;
}

void map_empty_static(Map *m)
{
  MAPZERO(m);
}
void map_set_static(Map *m, int n)
{
  MAPSET(m, n);
}
void map_setall_static(Map *m)
{
  MAPSETALL(m);
}
void map_clr_static(Map *m, int n)
{
  MAPCLR(m, n);
}
int map_tst_static(Map *m, int n)
{
  return MAPTST(m, n);
}
void map_clr_at_static(Map *m, int n)
{
  MAPCLR_AT(m, n);
}


// void rdnf_set
