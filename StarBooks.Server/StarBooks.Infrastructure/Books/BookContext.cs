using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Design;
using StarBooks.Domain.Books;

namespace StarBooks.Infrastructure.Books;

public class BookContext : DbContext
{
    public DbSet<BookModel> Books { get; set; }
    public DbSet<AuthorModel> Authors { get; set; }
    public DbSet<CategoryModel> Categories { get; set; }
    public DbSet<IndustryIdentifierModel> Identifiers { get; set; }

    public BookContext(DbContextOptions<BookContext> options) : base(options) { }
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
