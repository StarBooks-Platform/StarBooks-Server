using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Design;
using StarBooks.Domain.Books;

namespace StarBooks.Infrastructure.Books;

internal class BookContext : DbContext
{
    public DbSet<BookModel> Books { get; set; }

    public BookContext(DbContextOptions<BookContext> options) : base(options) { }

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        modelBuilder.Seed();
    }
}

internal class BookContextFactory : IDesignTimeDbContextFactory<BookContext>
{
    public BookContext CreateDbContext(string[] args)
    {
        var optionsBuilder = new DbContextOptionsBuilder<BookContext>();
        optionsBuilder.UseSqlServer("Server=localhost;Database=starbooks;Trusted_Connection=True;");

        return new BookContext(optionsBuilder.Options);
    }
}
