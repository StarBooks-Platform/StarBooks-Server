using Microsoft.EntityFrameworkCore;
using Refit;
using StarBooks.Domain.Books;
using StarBooks.Domain.Core.BooksDataSource;

namespace StarBooks.Infrastructure.Books;

internal static class ModelBuilderExtensions
{
    public static void Seed(this ModelBuilder modelBuilder)
    {
        var booksApi = RestService.For<IBooksApi>("https://www.googleapis.com/books/v1");
        var topics = new[] { "warrior-cats", "science-fiction", "programming" };

        topics
            .ToList()
            .ForEach(
                topic =>
                {
                    var books = booksApi.GetBooks(topic).Result.Items;
                    var nrOfBooks = books.Count;
                    books = books.Select(b =>
                    {
                        b.Id = nrOfBooks--;
                        return b;
                    }).ToList();
                    modelBuilder.Entity<BookModel>().HasData(books);
                }
            );
    }
}
